use axum::routing::get;

#[tokio::main]
async fn main() {
    // Build our application with a single route.
    let app = axum::Router::new()
        .route("/epoch",
            get(epoch)
        );

    // Run our application as a hyper server on http://localhost:3000.
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

/// axum handler for "GET /epoch" which shows the current epoch time.
/// This shows how to write a handler that uses time and can error.
pub async fn epoch() -> Result<String, axum::http::StatusCode> {
    match std::time::SystemTime::now().duration_since(std::time::SystemTime::UNIX_EPOCH) {
        Ok(duration) => Ok(format!("{}", duration.as_secs())),
        Err(_) => Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
    }
}
