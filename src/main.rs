// Use axum capabities.
use axum::{
    handler::Handler,
    http::StatusCode,
    http::Uri,
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
        .route("/demo.json", get(get_demo_json).put(put_demo_json))
        .route("/foo", get(get_foo).put(put_foo).patch(patch_foo).post(post_foo).delete(delete_foo))
        .route("/items", get(get_items))
        .route("/items/:id", get(get_items_id))
        .route("/books", get(get_books).put(put_books))
        .route("/books/:id", get(get_books_id))
        .route("/books/:id/form", get(get_books_id_form));

    // Run our application by using hyper and URL http://localhost:3000.
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

//// Demo axum handlers.

// axum handler for any request that fails to match the router routes.
// This implementation returns a HTTP status code 404 Not Found response.
async fn fallback(uri: Uri) -> impl axum::response::IntoResponse {
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
async fn get_demo_html() -> axum::response::Html<&'static str> {
    "<h1>Hello</h1>".into()
}

// axum handler for "GET /demo.json" which shows how to return JSON data.
// The `Json` type sets an HTTP header content-type of `application/json`.
// The `Json` type works with any type that implements `serde::Serialize`.
async fn get_demo_json() -> axum::extract::Json<Value> {
    json!({"a":"b"}).into()
}

// axum handler for "PUT /demo-json" which shows how to use `aumx::extract::Json`.
// This buffers the request body then deserializes it into a `serde_json::Value`.
// The axum `Json` type supports any type that implements `serde::Deserialize`.
async fn put_demo_json(axum::extract::Json(payload): axum::extract::Json<serde_json::Value>) -> String {
    format!("Put demo JSON payload: {:?}", payload)
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

// axum handler for "PATCH /foo" which shows naming convention for PATCH.
async fn patch_foo() -> String {
    "PATCH foo".to_string()
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
async fn get_items(axum::extract::Query(params): axum::extract::Query<HashMap<String, String>>) -> String {
    format!("Get items with query params: {:?}", params)
}

// axum handler for "GET /items/:id" which shows how to use `axum::extract::Path`.
// This extracts a path parameter then deserializes it as needed.
async fn get_items_id(axum::extract::Path(id): axum::extract::Path<String>) -> String {
    format!("Get items with path id: {:?}", id)
}

//// Demo books using a struct and lazy mutex global variable.

// Demo book structure with some example fields for id, title, author.
// A production app or database could use an id that is a u32, UUID, etc.
#[derive(Debug, Deserialize, Clone, Eq, Hash, PartialEq)]
struct Book {
    id: u32,
    title: String,
    author: String,
}

// Display the book using the format "{title} by {author}".
// This is a typical Rust trait and is not axum-specific.
impl std::fmt::Display for Book {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} by {}", self.title, self.author)
    }
}

// Use Deserialize to convert e.g. from request JSON into Book struct.
use serde::Deserialize;

// Use once_cell for creating a global variable e.g. our DATA data.
use once_cell::sync::Lazy;

// Use Mutex for thread-safe access to a variable e.g. our DATA data.
use std::sync::Mutex;

// Use Thread for spawning a thread e.g. to acquire our DATA mutex lock.
use std::thread;

// Create a data store as a global variable with `Lazy` and `Mutex`.
// This demo implementation uses a `HashMap` for ease and speed.
// The map key is a primary key for lookup; the map value is a Book.
//
// To access the data, the typical way is via a thread and lock:
//
// ```
// async fn demo() {
//     thread::spawn(move || {
//         let data = DATA.lock().unwrap();
//         …
// }).join().unwrap()
// ```
static DATA: Lazy<Mutex<HashMap<u32, Book>>> = Lazy::new(|| Mutex::new(
    HashMap::from([
        (1, Book { id: 1, title: "Antigone".into(), author: "Sophocles".into()}),
        (2, Book { id: 2, title: "Beloved".into(), author: "Toni Morrison".into()}),
        (3, Book { id: 3, title: "Candide".into(), author: "Voltaire".into()}),
    ])
));

// axum handler for "GET /books" which returns a resource index HTML page.
// This demo uses our DATA variable; a production app could use a database.
// This function needs to clone the DATA in order to sort them by title.
async fn get_books() -> axum::response::Html<String> {
    thread::spawn(move || {
        let data = DATA.lock().unwrap();
        let mut books = data.values().collect::<Vec<_>>().clone();
        books.sort_by(|a, b| a.title.cmp(&b.title));
        axum::response::Html(
            books.iter().map(|&book|
                format!("<p>{}</p>\n", &book)
            ).collect::<String>()
        )
    }).join().unwrap()
}

// axum handler for "PUT /books" which creates a new book resource.
// This demo shows how axum can extract a JSON payload into a Book struct.
async fn put_books(axum::extract::Json(book): axum::extract::Json<Book>) -> axum::response::Html<String> {
    DATA.lock().unwrap().insert(book.id, book.clone());
    format!("Put book: {}", &book).into()
}

// axum handler for "GET /books/:id" which returns one resource HTML page.
// This demo app uses our DATA data, and iterates on them to find the id.
async fn get_books_id(axum::extract::Path(id): axum::extract::Path<u32>) -> axum::response::Html<String> {
    match DATA.lock().unwrap().get(&id) {
        Some(book) => format!("<p>{}</p>\n", &book).into(),
        None => format!("<p>Book id {} not found</p>", id).into(),
    }
}

// axum handler for "GET /books/:id/form" which responds with an HTML form.
// This demo shows how to write a typical HTML form with input fields.
async fn get_books_id_form(axum::extract::Path(id): axum::extract::Path<u32>) -> axum::response::Html<String> {
    match DATA.lock().unwrap().get(&id) {
        Some(book) => format!(
            concat!(
                "<form method=\"post\" action=\"/books/{}/form\">\n",
                "<input type=\"hidden\" name=\"id\" value=\"{}\">\n",
                "<p><input type=\"text\" name=\"title\" value=\"{}\"></p>\n",
                "<p><input type=\"text\" name=\"author\" value=\"{}\"></p>\n",
                "<input type=\"submit\" value=\"Save\">\n",
                "</form>\n"
            ), 
            &book.id,
            &book.id,
            &book.title,
            &book.author
        ).into(),
        None => format!("<p>Book id {} not found</p>", id).into(),
    }
}
