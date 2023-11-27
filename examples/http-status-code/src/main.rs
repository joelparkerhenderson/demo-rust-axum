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
    let app = axum::Router::new()
        .route("/demo-http-status-code",
            get(demo_http_status_code)
        );

    // Run our application as a hyper server on http://localhost:3000.
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

/// axum handler for "GET /demo-http-status-code" which returns a 
/// HTTP status code, such as OK (200), and a visible string message.
pub async fn demo_http_status_code() -> (axum::http::StatusCode, String) {
    (axum::http::StatusCode::OK, "Everything is OK".to_string())
}
