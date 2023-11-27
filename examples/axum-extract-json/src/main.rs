//! Demo of Rust and axum web framework.
//!
//! <https://github.com/joelparkerhenderson/demo-rust-axum>
//!
//! For more see the file `README.md` in the project root.
//!
//! axum has capabilities for working with JSON data.
//!
//! The axum extractor for JSON can also help with a request, by deserializing a
//! request body into some type that implements serde::Deserialize. If the axum
//! extractor is unable to parse the request body, or the request does not
//! contain the Content-Type: application/json header, then the axum extractor
//! will reject the request and return a 400 Bad Request response.
//!
//! The axum extractor for JSON can help with a response, by formatting JSON data
//! then setting the response application content type.

/// Use axum capabilities.
use axum::routing::get;

/// Use Serde JSON to serialize/deserialize JSON, such as in a request.
/// axum creates JSON or extracts it by using `axum::extract::Json`.
/// For this demo, see functions `get_demo_json` and `post_demo_json`.
use serde_json::{json, Value};

#[tokio::main]
async fn main() {
    // Build our application by creating our router.
    let app = axum::Router::new().route(
        "/demo.json",
        get(get_demo_json)
            .put(put_demo_json)
    );

    // Run our application as a hyper server on http://localhost:3000.
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

/// axum handler for "GET /demo.json" which returns JSON data.
/// The `Json` type sets an HTTP header content-type `application/json`.
/// The `Json` type supports types that implement `serde::Deserialize`.
pub async fn get_demo_json() -> axum::extract::Json<Value> {
    json!({"a":"b"}).into()
}

/// axum handler for "PUT /demo.json" which uses `aumx::extract::Json`.
/// This buffers the request body then deserializes it using serde.
/// The `Json` type supports types that implement `serde::Deserialize`.
pub async fn put_demo_json(
    axum::extract::Json(data): axum::extract::Json<serde_json::Value>
) -> String {
    format!("Put demo JSON data: {:?}", data)
}
