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
use axum::routing::put;

/// Run our app using a hyper server on http://localhost:3000.
#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app()).await.unwrap();
}

/// Create our application.
pub fn app() -> axum::Router {
    axum::Router::new()
        .route("/demo.json",
            put(put_demo_json)
        )
}

/// axum handler for "PUT /demo.json" which uses `aumx::extract::Json`.
/// This buffers the request body then deserializes it using serde.
/// The `Json` type supports types that implement `serde::Deserialize`.
pub async fn put_demo_json(
    axum::extract::Json(data): axum::extract::Json<serde_json::Value>
) -> String {
    format!("Put demo JSON data: {:?}", data)
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum_test::TestServer;

    #[tokio::test]
    async fn test() {
        let server = TestServer::new(app()).unwrap();
        let j = serde_json::json!({"a":"b"});
        let response_text = server.put("/demo.json").json(&j).await.text();
        assert_eq!(response_text, "Put demo JSON data: Object {\"a\": String(\"b\")}");
    }
}
