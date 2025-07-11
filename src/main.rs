//! Demo of Rust and axum web framework.
//!
//! <https://github.com/joelparkerhenderson/demo-rust-axum>
//!
//! This demo shows how to:
//!
//! * Create a project using Rust and the axum web framework.
//!
//! * Create axum router routes and their handler functions.
//!
//! * Create responses with HTTP status code OK and HTML text.
//!
//! * Create a binary image and respond with a custom header.
//!
//! * Create functionality for HTTP GET, PUT, PATCH, POST, DELETE.
//!
//! * Use axum extractors for query parameters and path parameters.
//!
//! * Create a data store and access it using RESTful routes.
//!
//! For more see the file `README.md` in the project root.

pub mod app;

/// See file book.rs, which defines the `Book` struct.
mod book;

/// See file data.rs, which defines the DATA global variable.
mod data;

/// Use tracing crates for application-level tracing output.
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

/// The main function does these steps: 
/// - Start tracing and emit a tracing event.
/// - Get a command line argument as our bind address.
/// - Create our application which is an axum router/.
/// - Run our app using a hyper server.
#[tokio::main]  
async fn main() {
    // Start tracing and emit a tracing event.
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();
    tracing::event!(tracing::Level::INFO, "main");

    // Get command line arguments.
    let args: Vec<String> = std::env::args().skip(1).collect();

    // Use the first arg for tokio::net::TcpListener::bind(…)  
    let bind_address = match args.get(0) {
        Some(x) => x.clone(),
        None => "0.0.0.0:3000".into(),
    };

    // Create our application which is an axum router.
    let app = crate::app::app();

    // Run our app using a hyper server.
    let listener = tokio::net::TcpListener::bind(bind_address).await.unwrap();
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

/// Shutdown signal to run axum with graceful shutdown when
/// a user presses Ctrl+C or Unix sends a terminate signal.
pub async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}
