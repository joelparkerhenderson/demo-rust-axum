// Use axum capabities.
use axum::{
    handler::Handler,
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
        .route("/demo.png", get(get_demo_png))
        .route("/demo.json", get(get_demo_json).put(put_demo_json))
        .route("/foo", get(get_foo).put(put_foo).patch(patch_foo).post(post_foo).delete(delete_foo))
        .route("/items", get(get_items))
        .route("/items/:id", get(get_items_id))
        .route("/books", get(get_books).put(put_books))
        .route("/books/:id", get(get_books_id).delete(delete_books_id))
        .route("/books/:id/form", get(get_books_id_form).post(post_books_id_form));

    // Run our application by using hyper and URL http://localhost:3000.
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

//// Demo axum handlers.

// axum handler for any request that fails to match the router routes.
// This implementation returns a HTTP status code 404 Not Found response.
pub async fn fallback(uri: Uri) -> impl axum::response::IntoResponse {
    (axum::http::StatusCode::NOT_FOUND, format!("No route for {}", uri))
}

// axum handler for "GET /" which returns a string, which causes axum to
// immediately respond with a `200 OK` response, along with the plain text.
pub async fn hello() -> String {
    "Hello, World!".to_string()
}

// axum handler that implements `IntoResponse`, which allows us to return any
// HTTP status code (such as status code 200 OK) and any description string.
pub async fn demo_status() -> (axum::http::StatusCode, String) {
    (axum::http::StatusCode::OK, "Everything is OK".to_string())
}

// axum handler that implements `IntoResponse`, which allows us to return any
// HTTP status code (such as status code 200 OK) and any description string.
pub async fn demo_uri(uri: Uri) -> String {
    format!("The URI is: {:?}", uri)
}

// axum handler for "GET /demo.html" that responds with HTML text.
// The `Html` type sets an HTTP header content-type of `text/html`.
pub async fn get_demo_html() -> axum::response::Html<&'static str> {
    "<h1>Hello</h1>".into()
}

// axum handler for "GET /demo.png" that response with a PNG and header.
// This creates an image, then responds with a new header "image/png".
// Credit to https://github.com/ttys3/static-server/blob/main/src/main.rs
async fn get_demo_png() -> impl axum::response::IntoResponse {
    let png = "iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mPk+89QDwADvgGOSHzRgAAAAABJRU5ErkJggg==";
    let body = axum::body::Full::from(::base64::decode(png).unwrap());
    let mut response = axum::response::Response::new(body);
    response.headers_mut().insert(
        ::http::header::CONTENT_TYPE, 
        ::http::header::HeaderValue::from_static("image/png")
    );
    response
}

// axum handler for "GET /demo.json" which shows how to return JSON data.
// The `Json` type sets an HTTP header content-type of `application/json`.
// The `Json` type works with any type that implements `serde::Serialize`.
pub async fn get_demo_json() -> axum::extract::Json<Value> {
    json!({"a":"b"}).into()
}

// axum handler for "PUT /demo-json" which shows how to use `aumx::extract::Json`.
// This buffers the request body then deserializes it into a `serde_json::Value`.
// The axum `Json` type supports any type that implements `serde::Deserialize`.
pub async fn put_demo_json(axum::extract::Json(payload): axum::extract::Json<serde_json::Value>) -> String {
    format!("Put demo JSON payload: {:?}", payload)
}

//// Demo axum handlers with HTTP verbs GET, PUT, POST, DELETE.

// axum handler for "GET /foo" which shows naming convention for GET.
pub async fn get_foo() -> String {
    "GET foo".to_string()
}

// axum handler for "PUT /foo" which shows naming convention for PUT.
pub async fn put_foo() -> String {
    "PUT foo".to_string()
}

// axum handler for "PATCH /foo" which shows naming convention for PATCH.
pub async fn patch_foo() -> String {
    "PATCH foo".to_string()
}

// axum handler for "POST /foo" which shows naming convention for POST.
pub async fn post_foo() -> String {
    "POST foo".to_string()
}

// axum handler for "DELETE /foo" which shows naming convention for DELETE.
pub async fn delete_foo() -> String {
    "DELETE foo".to_string()
}

//// Demo axum handlers with extractors for query params and path params.

// axum handler for "GET /items" which shows how to use `axum::extrac::Query`.
// This extracts query parameters then deserializes them into a key-value map.
pub async fn get_items(axum::extract::Query(params): axum::extract::Query<HashMap<String, String>>) -> String {
    format!("Get items with query params: {:?}", params)
}

// axum handler for "GET /items/:id" which shows how to use `axum::extract::Path`.
// This extracts a path parameter then deserializes it as needed.
pub async fn get_items_id(axum::extract::Path(id): axum::extract::Path<String>) -> String {
    format!("Get items with path id: {:?}", id)
}

//// Demo books using a struct and lazy mutex global variable.

// See file book.rs, which defines the `Book` struct.
mod book;
use crate::book::Book;

// See file data.rs, which defines the DATA global variable.
mod data;
use crate::data::DATA;

// Use Thread for spawning a thread e.g. to acquire our crate::DATA mutex lock.
use std::thread;

// axum handler for "GET /books" which returns a resource index HTML page.
// This demo uses our crate::DATA variable; a production app could use a database.
// This function needs to clone the crate::DATA in order to sort them by title.
pub async fn get_books() -> axum::response::Html<String> {
    thread::spawn(move || {
        let data = DATA.lock().unwrap();
        let mut books = data.values().collect::<Vec<_>>().clone();
        books.sort_by(|a, b| a.title.cmp(&b.title));
        books.iter().map(|&book|
            format!("<p>{}</p>\n", &book)
        ).collect::<String>()
    }).join().unwrap().into()
}

// axum handler for "PUT /books" which creates a new book resource.
// This demo shows how axum can extract a JSON payload into a Book struct.
pub async fn put_books(axum::extract::Json(book): axum::extract::Json<Book>) -> axum::response::Html<String> {
    DATA.lock().unwrap().insert(book.id, book.clone());
    format!("Put book: {}", &book).into()
}

// axum handler for "GET /books/:id" which returns one resource HTML page.
// This demo app uses our crate::DATA variable, and iterates on it to find the id.
pub async fn get_books_id(axum::extract::Path(id): axum::extract::Path<u32>) -> axum::response::Html<String> {
    match DATA.lock().unwrap().get(&id) {
        Some(book) => format!("<p>{}</p>\n", &book),
        None => format!("<p>Book id {} not found</p>", id),
    }.into()
}

// axum handler for "DELETE /books/:id" which destroys an existing resource.
// This code shows how to extract an id, then mutate the crate::DATA variable.
pub async fn delete_books_id(axum::extract::Path(id): axum::extract::Path<u32>) -> axum::response::Html<String> {
    thread::spawn(move || {
        let mut data = DATA.lock().unwrap();
        if data.contains_key(&id) {
            data.remove(&id);
            format!("Delete book id: {}", &id)
        } else {
            format!("Book id not found: {}", &id)
        }
    }).join().unwrap().into()
}

// axum handler for "GET /books/:id/form" which responds with an HTML form.
// This demo shows how to write a typical HTML form with input fields.
pub async fn get_books_id_form(axum::extract::Path(id): axum::extract::Path<u32>) -> axum::response::Html<String> {
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
        ),
        None => format!("<p>Book id {} not found</p>", id),
    }.into()
}

// axum handler for "POST /books/:id/form" which submits an HTML form.
// This demo shows how to do a form submission then update a resource.
pub async fn post_books_id_form(form: axum::extract::Form<Book>) -> axum::response::Html<String> {
    let new_book: Book = form.0;
    thread::spawn(move || {
        let mut data = DATA.lock().unwrap();
        if data.contains_key(&new_book.id) {
            data.insert(new_book.id, new_book.clone());
            format!("Post book: {}", &new_book)
        } else {
            format!("Book id not found: {}", &new_book.id)
        }
    }).join().unwrap().into()
}
