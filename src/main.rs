extern crate axum;
extern crate tokio;

use axum::{
    extract::Json,
    extract::Path,
    extract::Query,
    handler::Handler,
    http::StatusCode,
    http::Uri,
    response::Html,
    response::IntoResponse,
    routing::get,
    Router,
};

use tracing_subscriber::{
    layer::SubscriberExt,
    util::SubscriberInitExt,
};

// Use HashMap to deserialize a HTTP GET query into a key-value map.
// Axum extracts query parameters by using `axum::extract::Query`.
// For the implementation, see function `get_query`.
use std::collections::HashMap;

// Use Serde JSON to serialize/deserialize JSON, such as the request body.
// Axum creates JSON payloads or extracts them by using `axum::extract::Json`.
// For the implementation, see functions `get_demo_json` and `post_demo_json`.
use serde_json::{json, Value};

#[tokio::main]
async fn main() {

    // Start tracing.
    tracing_subscriber::registry()
    .with(tracing_subscriber::fmt::layer())
    .init();

    // Build our application by creating our router and route.
    let app = Router::new()
        .fallback(fallback.into_service())
        .route("/", get(hello))
        .route("/demo-status", get(demo_status))
        .route("/demo-uri", get(demo_uri))
        .route("/demo.html", get(get_demo_html))
        .route("/demo.json", get(get_demo_json).post(post_demo_json))
        .route("/foo", get(get_foo).post(post_foo))
        .route("/item", get(get_item))
        .route("/item/:id", get(get_item_by_id));

    // Run our application by using hyper and URL http://localhost:3000.
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// Axum handler for any request that fails to match the router routes.
// This implementation returns a HTTP status code 404 Not Found response.
async fn fallback(uri: Uri) -> impl IntoResponse {
    (StatusCode::NOT_FOUND, format!("No route for {}", uri))
}

// Axum handler for "GET /" which returns a string, which causes Axum to
// immediately respond with a `200 OK` response, along with the plain text.
async fn hello() -> String {
    "Hello, World!".to_string()
}

// Axum handler that implements `IntoResponse`, which allows us to return any
// HTTP status code (such as status code 200 OK) and any description string.
async fn demo_status() -> (StatusCode, String) {
    (StatusCode::OK, "Everything is OK".to_string())
}

// Axum handler that implements `IntoResponse`, which allows us to return any
// HTTP status code (such as status code 200 OK) and any description string.
async fn demo_uri(uri: Uri) -> String {
    format!("The URI is: {:?}", uri)
}

// Axum handler for "GET /demo.html" which shows to return HTML text.
// The `Html` type sets an HTTP header content-type of `text/html`.
async fn get_demo_html() -> Html<&'static str> {
    Html("<h1>Hello</h1>")
}

// Axum handler for "GET /demo.json" which shows how to return JSON data.
// The `Json` type sets an HTTP header content-type of `application/json`.
// The `Json` type works with any type that implements `serde::Serialize`.
async fn get_demo_json() -> Json<Value> {
    Json(json!({"a":"b"}))
}

// Axum handler for "POST /demo-json" which shows how to use `aumx::extract::Json`.
// This buffers the request body then deserializes it into a `serde_json::Value`.
// The Axum `Json` type supports any type that implements `serde::Deserialize`.
async fn post_demo_json(Json(payload): Json<serde_json::Value>) -> String{
    format!("Get demo JSON payload: {:?}", payload)
}

// Axum handler for "GET /foo" which shows naming convention for GET.
async fn get_foo() -> String {
    "GET foo".to_string()
 }

// Axum handler for "POST /foo" which shows naming convention for POST.
async fn post_foo() -> String {
    "POST foo".to_string()
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
