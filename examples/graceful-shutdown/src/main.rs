//! Demo of Rust and axum web framework.
//!
//! <https://github.com/joelparkerhenderson/demo-rust-axum>
//!
//! For more see the file `README.md` in the project root.

/// Use axum capabilities.
use axum::routing::get;

#[tokio::main]
async fn main() {
    // Build our application by creating our router.
    let app = axum::Router::new();

    // Run our application as a hyper server on http://localhost:3000.
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await.unwrap();
}

/// Tokio signal handler that will wait for a user to press CTRL+C.
/// We use this in our hyper `Server` method `with_graceful_shutdown`.
async fn shutdown_signal() {
   tokio::signal::ctrl_c().await {
        Ok(()) => {},
        Err(err) => {
            eprintln!("Unable to listen for shutdown signal: {}", err);
        },
    }    
}
