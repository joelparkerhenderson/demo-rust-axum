// Use axum capabities.
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

// Use tracing crates for application-level tracing output.
use tracing_subscriber::{
    layer::SubscriberExt,
    util::SubscriberInitExt,
};

// Use HashMap to deserialize a HTTP GET query into a key-value map.
// axum extracts query parameters by using `axum::extract::Query`.
// For the implementation, see function `get_query`.
use std::collections::HashMap;

// Use Serde JSON to serialize/deserialize JSON, such as the request body.
// axum creates JSON payloads or extracts them by using `axum::extract::Json`.
// For the implementation, see functions `get_demo_json` and `post_demo_json`.
use serde_json::{json, Value};

#[tokio::main]
async fn main() {

    // Start tracing.
    tracing_subscriber::registry()
    .with(tracing_subscriber::fmt::layer())
    .init();

    // Build our application by creating our router.
    let app = Router::new()
        .fallback(fallback.into_service())
        .route("/", get(hello))
        .route("/demo-status", get(demo_status))
        .route("/demo-uri", get(demo_uri))
        .route("/demo.html", get(get_demo_html))
        .route("/demo.json", get(get_demo_json).post(post_demo_json))
        .route("/foo", get(get_foo).put(put_foo).post(post_foo).delete(delete_foo))
        .route("/items", get(get_items))
        .route("/items/:id", get(get_items_id))
        .route("/books", get(get_books))
        .route("/books/:id", get(get_books_id));

    // Run our application by using hyper and URL http://localhost:3000.
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();

}

//// Demo axum handlers.

// axum handler for any request that fails to match the router routes.
// This implementation returns a HTTP status code 404 Not Found response.
async fn fallback(uri: Uri) -> impl IntoResponse {
    (StatusCode::NOT_FOUND, format!("No route for {}", uri))
}

// axum handler for "GET /" which returns a string, which causes axum to
// immediately respond with a `200 OK` response, along with the plain text.
async fn hello() -> String {
    "Hello, World!".to_string()
}

// axum handler that implements `IntoResponse`, which allows us to return any
// HTTP status code (such as status code 200 OK) and any description string.
async fn demo_status() -> (StatusCode, String) {
    (StatusCode::OK, "Everything is OK".to_string())
}

// axum handler that implements `IntoResponse`, which allows us to return any
// HTTP status code (such as status code 200 OK) and any description string.
async fn demo_uri(uri: Uri) -> String {
    format!("The URI is: {:?}", uri)
}

// axum handler for "GET /demo.html" which shows to return HTML text.
// The `Html` type sets an HTTP header content-type of `text/html`.
async fn get_demo_html() -> Html<&'static str> {
    Html("<h1>Hello</h1>")
}

// axum handler for "GET /demo.json" which shows how to return JSON data.
// The `Json` type sets an HTTP header content-type of `application/json`.
// The `Json` type works with any type that implements `serde::Serialize`.
async fn get_demo_json() -> Json<Value> {
    Json(json!({"a":"b"}))
}

// axum handler for "POST /demo-json" which shows how to use `aumx::extract::Json`.
// This buffers the request body then deserializes it into a `serde_json::Value`.
// The axum `Json` type supports any type that implements `serde::Deserialize`.
async fn post_demo_json(Json(payload): Json<serde_json::Value>) -> String{
    format!("Get demo JSON payload: {:?}", payload)
}

//// Demo axum handlers with HTTP verbs GET, PUT, POST, DELETE.

// axum handler for "GET /foo" which shows naming convention for GET.
async fn get_foo() -> String {
    "GET foo".to_string()
}

// axum handler for "PUT /foo" which shows naming convention for PUT.
async fn put_foo() -> String {
    "PUT foo".to_string()
}

// axum handler for "POST /foo" which shows naming convention for POST.
async fn post_foo() -> String {
    "POST foo".to_string()
}

// axum handler for "DELETE /foo" which shows naming convention for DELETE.
async fn delete_foo() -> String {
    "DELETE foo".to_string()
}

//// Demo axum handlers with extractors for query params and path params.

// axum handler for "GET /items" which shows how to use `axum::extrac::Query`.
// This extracts query parameters then deserializes them into a key-value map.
async fn get_items(Query(params): Query<HashMap<String, String>>) -> String {
    format!("Get items with query params: {:?}", params)
}

// axum handler for "GET /items/:id" which shows how to use `axum::extract::Path`.
// This extracts a path parameter then deserializes it into an integer.
async fn get_items_id(Path(id): Path<u32>) -> String {
    format!("Get items with path id: {:?}", id)
}

//// Demo books using a struct and lazy mutex global variable.

// Demo book structure that we can debug, clone, hash, and compare.
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Book {
    id: u32,
    title: String,
    author: String,
}

// Use once_cell for creating a global variable e.g. our BOOKS data.
use once_cell::sync::Lazy;

// Use Mutex for thread-safe access to a variable e.g. our BOOKS data.
use std::sync::Mutex;

// Use HashSet for a collection of items e.g. our BOOKS data.
use std::collections::HashSet;

// Create a data store as a global variable by using `once_cell` and `Mutex`.
static BOOKS: Lazy<Mutex<HashSet<Book>>> = Lazy::new(|| Mutex::new({
    let vec = vec![
        Book { id: 1, title: "Antigone".into(), author: "Sophocles".into()},
        Book { id: 2, title: "Beloved".into(), author: "Toni Morrison".into()},
        Book { id: 3, title: "Candide".into(), author: "Voltaire".into()},
    ];
    vec.into_iter().collect::<HashSet<_>>()
}));

// axum handler for "GET /books" which returns a resource index HTML page.
// This demo app uses our BOOKS data; a production app could use a database.
// This function needs to clone the BOOKS in order to sort them by title.
async fn get_books() -> Html<String> {
    let mut books = Vec::from_iter(BOOKS.lock().unwrap().clone());
    books.sort_by(|a, b| a.title.cmp(&b.title));
    Html(
        books.iter().map(|book|
            format!(
                "<p>{} by {}</p>\n",
                &book.title,
                &book.author
            )
        ).collect::<String>()
    )
}

// axum handler for "GET /books/:id" which returns one resource HTML page.
// This demo app uses our BOOKS data, and iterates on them to find the id.
async fn get_books_id(Path(id): Path<u32>) -> Html<String> {
    match BOOKS.lock().unwrap().iter().find(|&book| &book.id == &id) {
        Some(book) => Html(
            format!(
                "<p>{} by {}</p>\n",
                &book.title,
                &book.author
            )
        ),
        None => Html("<p>Not found</p>".into()),
    }
}
