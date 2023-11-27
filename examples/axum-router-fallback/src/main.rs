//! Demo of Rust and axum web framework.
//!
//! <https://github.com/joelparkerhenderson/demo-rust-axum>
//!
//! For more see the file `README.md` in the project root.

#[tokio::main]
async fn main() {
    // Build our application by creating our router.
    let app = axum::Router::new()
        .fallback(
            fallback
        );

    // Run our application as a hyper server on http://localhost:3000.
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

/// axum handler for any request that fails to match the router routes.
/// This implementation responds with HTTP status code NOT FOUND (404).
pub async fn fallback(
    uri: axum::http::Uri
) -> impl axum::response::IntoResponse {
    (
        axum::http::StatusCode::NOT_FOUND,
        format!("No route {}", uri)
    )
}
