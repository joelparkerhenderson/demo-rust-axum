//! Demo of Rust and axum web framework.
//!
//! <https://github.com/joelparkerhenderson/demo-rust-axum>
//!
//! For more see the file `README.md` in the project root.

/// Use axum capabilities.
use axum::routing::get;

#[tokio::main]
async fn main() {
    // Build our application by creating our router.
    let app = axum::Router::new()
        .route("/demo-path/:id",
            get(get_demo_path_id)
        );

    // Run our application as a hyper server on http://localhost:3000.
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

/// axum handler for "GET /demo-path/:id" which uses `axum::extract::Path`.
/// This extracts a path parameter then deserializes it as needed.
pub async fn get_demo_path_id(
    axum::extract::Path(id):
        axum::extract::Path<String>
) -> String {
    format!("Get demo path id: {:?}", id)
}
