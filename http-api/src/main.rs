//! `zpl-server`: serves the rendering API. Listens on `ZPL_API_ADDR` (default
//! `127.0.0.1:3000`). API keys and usage reporting: see `zpl_http_api::billing`.

#[tokio::main]
async fn main() {
    let addr = std::env::var("ZPL_API_ADDR").unwrap_or_else(|_| "127.0.0.1:3000".to_string());

    let billing = zpl_http_api::billing::Billing::from_env().unwrap_or_else(|e| {
        eprintln!("{e}");
        std::process::exit(1);
    });
    let (billing, publisher) = match billing {
        Some((billing, publisher)) => (Some(billing), Some(publisher)),
        None => {
            println!("API keys are off (ZPL_KEYS_PUBLIC_KEYS is not set)");
            (None, None)
        }
    };

    zpl_http_api::warm_up();

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap_or_else(|e| {
        eprintln!("Failed to bind {addr}: {e}");
        std::process::exit(1);
    });
    println!("Listening on http://{addr}");

    let router = zpl_http_api::router_with(&zpl_http_api::playground_dir(), billing);
    axum::serve(listener, router)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .expect("server error");

    // The router (and with it the usage sender) is gone: let the publisher send what is
    // still queued, within the shutdown grace period.
    if let Some(publisher) = publisher {
        let _ = tokio::time::timeout(std::time::Duration::from_secs(10), publisher).await;
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
