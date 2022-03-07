extern crate axum;
extern crate tokio;

use axum::{
    routing::get,
    Router,
};

#[tokio::main]
async fn main() {
    // Build our application by creating our router and route.
    let app = Router::new()
        .route("/", get(hello))
        .route("/foo", get(get_foo).post(post_foo))
        .route("/bar", get(get_bar).post(post_bar));

    // Run our application by using hyper and URL http://localhost:3000.
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}


// Axum handlers that immediately return an empty `200 OK` response,
// that return a typical string, and cause a plain text response.

async fn hello() -> String {
    "Hello, World!".to_string()
}

async fn get_foo() -> String {
    "Get Foo!".to_string()
 }
 
 async fn post_foo() -> String {
    "Post Foo!".to_string()
 }
 
 async fn get_bar() -> String {
    "Get Bar!".to_string()
 }
 
 async fn post_bar() -> String {
    "Post Bar!".to_string()
 }
 