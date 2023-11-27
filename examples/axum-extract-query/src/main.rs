//! Demo of Rust and axum web framework.
//!
//! <https://github.com/joelparkerhenderson/demo-rust-axum>
//!
//! For more see the file `README.md` in the project root.

/// Use axum capabilities.
use axum::routing::get;

/// Use HashMap to deserialize a HTTP GET query into a key-value map.
/// axum extracts query parameters by using `axum::extract::Query`.
/// For the implementation, see function `get_query`.
use std::collections::HashMap;

#[tokio::main]
async fn main() {
    // Build our application by creating our router.
    let app = axum::Router::new()
        .route("/demo-query",
            get(get_demo_query)
        );

    // Run our application as a hyper server on http://localhost:3000.
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

/// axum handler for "GET /demo-query" which uses `axum::extract::Query`.
/// This extracts query parameters and creates a key-value pair map.
pub async fn get_demo_query(
    axum::extract::Query(params):
        axum::extract::Query<HashMap<String, String>>
) -> String {
    format!("Demo query params: {:?}", params)
}
