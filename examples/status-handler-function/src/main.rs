use axum::routing::get;

#[tokio::main]
async fn main() {
    // Build our application with a single route.
    let app = axum::Router::new()
        .route("/status",
            get(status)
        );

    // Run our application as a hyper server on http://localhost:3000.
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

/// axum handler for "GET /status" which returns the HTTP status
/// code OK (200) along with a user-visible string message.
pub async fn status() -> (axum::http::StatusCode, String) {
    (axum::http::StatusCode::OK, "OK".to_string())
}
