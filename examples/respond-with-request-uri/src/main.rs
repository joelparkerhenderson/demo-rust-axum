use axum::routing::get;

#[tokio::main]
async fn main() {
    // Build our application with a single route.
    let app = axum::Router::new()
        .route("/request-uri",
            get(request_uri)
        );

    // Run our application as a hyper server on http://localhost:3000.
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

/// axum handler for "GET /request-uri" which shows the request's own URI.
/// This shows how to write a handler that receives the URI.
pub async fn request_uri(uri: axum::http::Uri) -> String {
    format!("The URI is: {:?}", uri)
}
