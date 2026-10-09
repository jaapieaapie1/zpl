//! `zpl-server`: serves the rendering API. Listens on `ZPL_API_ADDR` (default
//! `127.0.0.1:3000`).

#[tokio::main]
async fn main() {
    let addr = std::env::var("ZPL_API_ADDR").unwrap_or_else(|_| "127.0.0.1:3000".to_string());

    zpl_http_api::warm_up();

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap_or_else(|e| {
        eprintln!("Failed to bind {addr}: {e}");
        std::process::exit(1);
    });
    println!("Listening on http://{addr}");

    axum::serve(listener, zpl_http_api::router())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .expect("server error");
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
