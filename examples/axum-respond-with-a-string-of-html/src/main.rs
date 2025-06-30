use axum::routing::get;

#[tokio::main]
async fn main() {
    // Build our application with a single route.
    let app = axum::Router::new()
        .route("/string.html",
            get(string_html)
        );

    // Run our application as a hyper server on http://localhost:3000.
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

/// axum handler for "GET /string.html" which responds with a string.
/// The `Html` type sets an HTTP header content-type of `text/html`.
pub async fn string_html() -> axum::response::Html<&'static str> {
    "<html><body><h1>Headline</h1><p>Paragraph</b></body></html>".into()
}
