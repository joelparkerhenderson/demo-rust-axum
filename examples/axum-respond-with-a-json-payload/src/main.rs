use axum::routing::get;

#[tokio::main]
async fn main() {
    // Build our application with a single route.
    let app = axum::Router::new()
        .route("/demo.json",
            get(get_demo_json)
        );

    // Run our application as a hyper server on http://localhost:3000.
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

/// axum handler for "GET /demo.json" which returns JSON data.
/// The `Json` type sets an HTTP header content-type `application/json`.
/// The `Json` type supports types that implement `serde::Deserialize`.
pub async fn get_demo_json() -> axum::extract::Json<serde_json::Value> {
    serde_json::json!({"a":"b"}).into()
}
