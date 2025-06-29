use axum::routing::get;

#[tokio::main]
async fn main() {
    // Build our application with a single route.
    let app = axum::Router::new()
        .route("/demo.png",
            get(get_demo_png)
        );

    // Run our application as a hyper server on http://localhost:3000.
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

/// axum handler for "GET /demo.png" which responds with an image PNG.
/// This sets a header "image/png" then sends the decoded image data.
async fn get_demo_png() -> impl axum::response::IntoResponse {
    use base64::Engine;
    let png = concat!(
        "iVBORw0KGgoAAAANSUhEUgAAAAEAAAAB",
        "CAYAAAAfFcSJAAAADUlEQVR42mPk+89Q",
        "DwADvgGOSHzRgAAAAABJRU5ErkJggg=="
    );
    (
        axum::response::AppendHeaders([
            (axum::http::header::CONTENT_TYPE, "image/png"),
        ]),
        base64::engine::general_purpose::STANDARD.decode(png).unwrap(),
    )
}
