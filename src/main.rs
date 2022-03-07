extern crate axum;
extern crate tokio;

use axum::{
    routing::get,
    Router,
};

// Axum handlers that immediately return an empty `200 OK` response,
// that return a typical string, and cause a plain text response.

async fn hello() -> String {
    "Hello, World!".to_string()
}

async fn foo() -> String {
   "Foo!".to_string()
}

async fn bar() -> String {
   "Bar!".to_string()
}

#[tokio::main]
async fn main() {
    // Build our application by creating our router and route.
    let app = Router::new()
        .route("/", get(hello))
        .route("/foo", get(foo))
        .route("/bar", get(bar));

    // Run our application by using hyper and URL http://localhost:3000.
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
