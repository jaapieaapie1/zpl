//! `zpl-server`: serves the rendering API. Listens on `ZPL_API_ADDR` (default
//! `127.0.0.1:3000`). API keys and usage reporting: see `zpl_http_api::billing`; render
//! concurrency and load shedding: see `zpl_http_api::admission`.

#[tokio::main]
async fn main() {
    let addr = std::env::var("ZPL_API_ADDR").unwrap_or_else(|_| "127.0.0.1:3000".to_string());

    let billing = zpl_http_api::billing::Billing::from_env().unwrap_or_else(|e| {
        eprintln!("{e}");
        std::process::exit(1);
    });
    let (billing, publisher) = match billing {
        Some((billing, publisher)) => (Some(billing), publisher),
        None => {
            println!("API keys are off (ZPL_KEYS_PUBLIC_KEYS is not set)");
            (None, None)
        }
    };

    let admission = zpl_http_api::admission::Admission::from_env().unwrap_or_else(|e| {
        eprintln!("{e}");
        std::process::exit(1);
    });
    println!(
        "Rendering {} at once; up to {} requests wait, each at most {} ms",
        admission.concurrency(),
        admission.max_queue(),
        admission.max_wait().as_millis()
    );

    zpl_http_api::warm_up();

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap_or_else(|e| {
        eprintln!("Failed to bind {addr}: {e}");
        std::process::exit(1);
    });
    println!("Listening on http://{addr}");

    let router =
        zpl_http_api::router_with_admission(&zpl_http_api::playground_dir(), billing, std::sync::Arc::new(admission));
    axum::serve(listener, router)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .expect("server error");

    // Every request is done: publish the usage still adding up, within the shutdown grace
    // period (45 s in Kubernetes, 10 s of it in preStop).
    if let Some(publisher) = publisher {
        publisher.finish(std::time::Duration::from_secs(20)).await;
    }
}

/// Resolves on Ctrl-C or, on Unix, SIGTERM (what Kubernetes and Docker send on stop).
async fn shutdown_signal() {
    let ctrl_c = async {
        let _ = tokio::signal::ctrl_c().await;
    };
    #[cfg(unix)]
    let terminate = async {
        match tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate()) {
            Ok(mut sigterm) => {
                sigterm.recv().await;
            }
            Err(_) => std::future::pending::<()>().await,
        }
    };
    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {}
        _ = terminate => {}
    }
}
