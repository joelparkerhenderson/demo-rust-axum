extern crate axum;
extern crate tokio;

use axum::{
    extract::Path,
    routing::get,
    Router,
};

#[tokio::main]
async fn main() {
    // Build our application by creating our router and route.
    let app = Router::new()
        .route("/", get(hello))
        .route("/foo", get(get_foo).post(post_foo))
        .route("/bar", get(get_bar).post(post_bar))
        .route("/item/:id", get(get_item));
    
    // Run our application by using hyper and URL http://localhost:3000.
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}


// Axum handler for "GET /" which returns a string, which causes Axum to
// immediately respond with a `200 OK` response, along with the plain text.
async fn hello() -> String {
    "Hello, World!".to_string()
}

// Axum handler for "GET /foo"
async fn get_foo() -> String {
    "Get Foo!".to_string()
 }
 
// Axum handler for "POST /foo"
async fn post_foo() -> String {
    "Post Foo!".to_string()
}
 
// Axum handler for "GET /bar"
async fn get_bar() -> String {
    "Get Bar!".to_string()
}
 
// Axum handler for "POST /bar"
async fn post_bar() -> String {
    "Post Bar!".to_string()
}
 
// Axum handler for "GET /item/:id" which shows how to use `axum::extract::Path`
// in order to extract a path parameter and deserialize it to an integer.
async fn get_item(Path(id): Path<u32>) -> String {
    format!("Get item with identifier {:?}", id).to_string()
}
