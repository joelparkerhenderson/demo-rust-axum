extern crate axum;
extern crate tokio;

use axum::{
    extract::Json,
    extract::Path,
    extract::Query,
    routing::get,
    Router,
};

// Use HashMap to deserialize a HTTP GET query into a key-value map.
// Axum extracts query parameters by using `axum::extract::Query`.
// For the implementation, see the function `get_query`.
use std::collections::HashMap;

#[tokio::main]
async fn main() {
    // Build our application by creating our router and route.
    let app = Router::new()
        .route("/", get(hello))
        .route("/foo", get(get_foo).post(post_foo))
        .route("/bar", get(get_bar).post(post_bar))
        .route("/item", get(get_item))
        .route("/item/:id", get(get_item_by_id))
        .route("/demo-json", get(get_demo_json));      
    
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

// Axum handler for "GET /item" which shows how to use `axum::extrac::Query`.
// This extracts query parameters then deserializes them into a key-value map.
async fn get_item(Query(params): Query<HashMap<String, String>>) -> String {
    format!("Get item query params: {:?}", params)
}

// Axum handler for "GET /item/:id" which shows how to use `axum::extract::Path`.
// This extracts a path parameter then deserializes it into an integer.
async fn get_item_by_id(Path(id): Path<u32>) -> String {
    format!("Get item by id: {:?}", id)
}

// Axum handler for "GET /json" which shows how to use `aumx::extract::Json`.
// This buffers the request body then deserializes it into a `serde_json::Value`.
// The Axum `Json` type supports any type that implements `serde::Deserialize`.
async fn get_demo_json(Json(payload): Json<serde_json::Value>) -> String{
    format!("Get demo JSON payload: {:?}", payload)
}
