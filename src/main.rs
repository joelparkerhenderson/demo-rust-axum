extern crate axum;
extern crate tokio;

use axum::{
    routing::get,
    Router,
};

// Axum handler that immediately returns an empty `200 OK` response,
// with a plain text body that will reponds with "hello world".
async fn hello_handler() -> String {
    "Hello, World!".to_string()
}

#[tokio::main]
async fn main() {
    // Build our application by creating our router and route.
    let app = Router::new()
        .route("/", get(hello_handler));

    // Run our application by using hyper and URL http://localhost:3000.
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
