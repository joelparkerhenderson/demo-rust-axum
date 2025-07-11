/// Use axum capabilities.
use axum::routing::*;

/// Use HashMap to deserialize a HTTP GET query into a key-value map.
/// axum extracts query parameters by using `axum::extract::Query`.
/// For the implementation, see function `get_query`.
use std::collections::HashMap;

/// Use Serde JSON to serialize/deserialize JSON, such as in a request.
/// axum creates JSON or extracts it by using `axum::extract::Json`.
/// For this demo, see functions `get_demo_json` and `put_demo_json`.
use serde_json::{json, Value};

/// Create our application which is an axum router.
pub fn app() -> axum::Router {
    axum::Router::new()
        .fallback(fallback)
        .route("/", get(hello))
        .route("/string.html", get(string_html))
        .route("/file.html", get(file_html))
        .route("/status", get(status))
        .route("/epoch", get(epoch))
        .route("/uptime", get(uptime))
        .route("/count", get(count))
        .route("/request-uri", get(request_uri))
        .route("/demo.html", get(demo_html))
        .route("/demo.png", get(demo_png))
        .route("/demo.json", get(get_demo_json).put(put_demo_json))
        .route(
            "/foo",
            get(get_foo)
                .put(put_foo)
                .patch(patch_foo)
                .post(post_foo)
                .delete(delete_foo),
        )
        .route("/items", get(get_items))
        .route("/items/{id}", get(get_items_id))
        .route("/books", get(get_books).put(put_books))
        .route("/books/{id}", get(get_books_id).delete(delete_books_id))
        .route(
            "/books/{id}/form",
            get(get_books_id_form).post(post_books_id_form),
        )
}

////
// Demo axum handlers
//
// These handlers are used to demonstrate axum capabilities.
// Each handler is an async function that returns something that
// axum can convert into a response.
////

/// axum handler for any request that fails to match the router routes.
/// This implementation returns HTTP status code Not Found (404).
pub async fn fallback(uri: axum::http::Uri) -> impl axum::response::IntoResponse {
    (axum::http::StatusCode::NOT_FOUND, uri.to_string())
}

/// axum handler for "GET /" which returns a string and causes axum to
/// immediately respond with status code `200 OK` and with the string.
pub async fn hello() -> String {
    "Hello, World!".to_string()
}

/// axum handler for "GET /string.html" which responds with a string.
/// The `Html` type sets an HTTP header content-type of `text/html`.
pub async fn string_html() -> axum::response::Html<&'static str> {
    "<html><body><h1>Headline</h1><p>Paragraph</b></body></html>".into()
}

/// axum handler that responds with typical HTML coming from a file.
/// This uses the Rust macro `std::include_str` to include a UTF-8 file
/// path, relative to `main.rs`, as a `&'static str` at compile time.
async fn file_html() -> axum::response::Html<&'static str> {
    include_str!("file.html").into()
}

/// axum handler for "GET /status" which returns the HTTP status
/// code OK (200) along with a user-visible string message.
pub async fn status() -> (axum::http::StatusCode, String) {
    (axum::http::StatusCode::OK, "OK".to_string())
}

////
// This section is much the same as the repo `web-service-epoch-axum`.
////

/// axum handler for "GET /epoch" which shows the current epoch time.
/// This shows how to write a handler that uses time and can error.
pub async fn epoch() -> Result<String, axum::http::StatusCode> {
    match std::time::SystemTime::now().duration_since(std::time::SystemTime::UNIX_EPOCH) {
        Ok(duration) => Ok(format!("{}", duration.as_secs())),
        Err(_) => Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
    }
}

////
// This section is much the same as the repo `web-service-uptime-axum`.
////

/// Create the constant INSTANT so the program can track its own uptime.
pub static INSTANT: std::sync::LazyLock<std::time::Instant> = std::sync::LazyLock::new(|| std::time::Instant::now());

/// axum handler for "GET /uptime" which shows the program's uptime duration.
/// This shows how to write a handler that uses a global static lazy value.
pub async fn uptime() -> String {
    format!("{}", INSTANT.elapsed().as_secs())
}

////
// This section is much the same as the repo `web-service-count-axum`.
////

/// Create the atomic variable COUNT so the program can track its own count.
pub static COUNT: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);

/// axum handler for "GET /count" which shows the program's count duration.
/// This shows how to write a handler that uses a global static lazy value.
pub async fn count() -> String {
    COUNT.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    format!("{}", COUNT.load(std::sync::atomic::Ordering::SeqCst))
}

////

/// axum handler for "GET /request-uri" which shows the request's own URI.
/// This shows how to write a handler that receives the URI.
pub async fn request_uri(uri: axum::http::Uri) -> String {
    format!("The URI is: {:?}", uri)
}


/// axum handler for "GET /demo.html" which responds with HTML text.
/// The `Html` type sets an HTTP header content-type of `text/html`.
pub async fn demo_html() -> axum::response::Html<&'static str> {
    "<h1>Hello</h1>".into()
}

/// axum handler for "GET /demo.png" which responds with an image PNG.
/// This sets a header "image/png" then sends the decoded image data.
async fn demo_png() -> impl axum::response::IntoResponse {
    use base64::Engine;
    let png = "iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mPk+89QDwADvgGOSHzRgAAAAABJRU5ErkJggg==";
    (
        axum::response::AppendHeaders([(axum::http::header::CONTENT_TYPE, "image/png")]),
        base64::engine::general_purpose::STANDARD
            .decode(png)
            .unwrap(),
    )
}

////
// Demo axum JSON extractor
//
// axum has capabilities for working with JSON data.
//
// The axum extractor for JSON can also help with a request, by deserializing a
// request body into some type that implements serde::Deserialize. If the axum
// extractor is unable to parse the request body, or the request does not
// contain the Content-Type: application/json header, then the axum extractor
// will reject the request and return a 400 Bad Request response.
//
// The axum extractor for JSON can help with a response, by formatting JSON data
// then setting the response application content type.
////

/// axum handler for "GET /demo.json" which returns JSON data.
/// The `Json` type sets an HTTP header content-type `application/json`.
/// The `Json` type supports types that implement `serde::Deserialize`.
pub async fn get_demo_json() -> axum::extract::Json<Value> {
    json!({"a":"b"}).into()
}

/// axum handler for "PUT /demo.json" which uses `aumx::extract::Json`.
/// This buffers the request body then deserializes it using serde.
/// The `Json` type supports types that implement `serde::Deserialize`.
pub async fn put_demo_json(
    axum::extract::Json(data): axum::extract::Json<serde_json::Value>,
) -> String {
    format!("Put demo JSON data: {:?}", data)
}

////
// Demo axum handlers with HTTP verbs GET, PUT, PATCH, POST, DELETE.
//
// The axum route has functions for each HTTP verb.
// We use the naming convention of `get_foo`, `put_foo`, etc.
//
// These demo handlers in this section simply return a string.
// After this section, you'll see how to return other data.
////

/// axum handler for "GET /foo" which returns a string message.
/// This shows our naming convention for HTTP GET handlers.
pub async fn get_foo() -> String {
    "GET foo".to_string()
}

/// axum handler for "PUT /foo" which returns a string message.
/// This shows our naming convention for HTTP PUT handlers.
pub async fn put_foo() -> String {
    "PUT foo".to_string()
}

/// axum handler for "PATCH /foo" which returns a string message.
/// This shows our naming convention for HTTP PATCH handlers.
pub async fn patch_foo() -> String {
    "PATCH foo".to_string()
}

/// axum handler for "POST /foo" which returns a string message.
/// This shows our naming convention for HTTP POST handlers.
pub async fn post_foo() -> String {
    "POST foo".to_string()
}

/// axum handler for "DELETE /foo" which returns a string message.
/// This shows our naming convention for HTTP DELETE handlers.
pub async fn delete_foo() -> String {
    "DELETE foo".to_string()
}

////
// Demo axum handlers with extractors for query params and path params.
//
// axum can automatically extract parameters from a request,
// and then pass them to a handler, using function parameters.
////

/// axum handler for "GET /items" which uses `axum::extract::Query`.
/// This extracts query parameters and creates a key-value pair map.
pub async fn get_items(
    axum::extract::Query(params): axum::extract::Query<HashMap<String, String>>,
) -> String {
    format!("Get items with query params: {:?}", params)
}

/// axum handler for "GET /items/{id}" which uses `axum::extract::Path`.
/// This extracts a path parameter then deserializes it as needed.
pub async fn get_items_id(axum::extract::Path(id): axum::extract::Path<String>) -> String {
    format!("Get items with path id: {:?}", id)
}

/////
// Demo books using RESTful routes and a data store.
//
// This section uses a `Book` struct, a `DATA` variable
// that is a lazy mutex global variable, and handlers
// that process the routes for HTTP verbs GET, PUT, etc.
/////

/// See file book.rs, which defines the `Book` struct.
use crate::book::Book;

/// See file data.rs, which defines the DATA global variable.
use crate::data::DATA;

/// Use Thread for spawning a thread e.g. to acquire our crate::DATA mutex lock.
use std::thread;

/// To access data, create a thread, spawn it, then get the lock.
/// When you're done, then join the thread with its parent thread.
#[allow(dead_code)]
async fn print_data() {
    thread::spawn(move || {
        match DATA.lock() {
            Ok(data) => {
        println!("data: {:?}", data);
    })
    .join()
    .unwrap()
}

/// axum handler for "GET /books" which responds with a resource page.
/// This demo uses our DATA; a production app could use a database.
/// This demo must clone the DATA in order to sort items by title.
pub async fn get_books() -> axum::response::Html<String> {
    thread::spawn(move || {
        match DATA.lock() {
            Ok(data) => {
        let mut books = data.values().collect::<Vec<_>>().clone();
        books.sort_by(|a, b| a.title.cmp(&b.title));
        books
            .iter()
            .map(|&book| format!("<p>{}</p>\n", &book))
            .collect::<String>()
    })
    .join()
    .unwrap()
    .into()
}

/// axum handler for "PUT /books" which creates a new book resource.
/// This demo shows how axum can extract JSON data into a Book struct.
pub async fn put_books(
    axum::extract::Json(book): axum::extract::Json<Book>,
) -> axum::response::Html<String> {
    thread::spawn(move || {
        match DATA.lock() {
            Ok(mut data) => {
        data.insert(book.id, book.clone());
        format!("Put book: {}", &book)
    })
    .join()
    .unwrap()
    .into()
}

/// axum handler for "GET /books/{id}" which responds with one resource HTML page.
/// This demo app uses our crate::DATA variable, and iterates on it to find the id.
pub async fn get_books_id(
    axum::extract::Path(id): axum::extract::Path<u32>,
) -> axum::response::Html<String> {
    thread::spawn(move || {
        match DATA.lock() {
            Ok(data) => {
        match data.get(&id) {
            Some(book) => format!("<p>{}</p>\n", &book),
            None => format!("<p>Book id {} not found</p>", id),
        }
    })
    .join()
    .unwrap()
    .into()
}

/// axum handler for "DELETE /books/{id}" which destroys a resource.
/// This demo extracts an id, then mutates the book in the DATA store.
pub async fn delete_books_id(
    axum::extract::Path(id): axum::extract::Path<u32>,
) -> axum::response::Html<String> {
    thread::spawn(move || {
        match DATA.lock() {
            Ok(mut data) => {
        if data.contains_key(&id) {
            data.remove(&id);
            format!("Delete book id: {}", &id)
        } else {
            format!("Book id not found: {}", &id)
        }
    })
    .join()
    .unwrap()
    .into()
}

/// axum handler for "GET /books/{id}/form" which responds with a form.
/// This demo shows how to write a typical HTML form with input fields.
pub async fn get_books_id_form(
    axum::extract::Path(id): axum::extract::Path<u32>,
) -> axum::response::Html<String> {
    thread::spawn(move || {
        match DATA.lock() {
            Ok(data) => {
        match data.get(&id) {
            Some(book) => format!(
                concat!(
                    "<form method=\"post\" action=\"/books/{}/form\">\n",
                    "<input type=\"hidden\" name=\"id\" value=\"{}\">\n",
                    "<p><input name=\"title\" value=\"{}\"></p>\n",
                    "<p><input name=\"author\" value=\"{}\"></p>\n",
                    "<input type=\"submit\" value=\"Save\">\n",
                    "</form>\n"
                ),
                &book.id, &book.id, &book.title, &book.author
            ),
            None => format!("<p>Book id {} not found</p>", id),
        }
    })
    .join()
    .unwrap()
    .into()
}

/// axum handler for "POST /books/{id}/form" which submits an HTML form.
/// This demo shows how to do a form submission then update a resource.
pub async fn post_books_id_form(form: axum::extract::Form<Book>) -> axum::response::Html<String> {
    let new_book: Book = form.0;
    thread::spawn(move || {
        match DATA.lock() {
            Ok(mut data) => {
        if data.contains_key(&new_book.id) {
            data.insert(new_book.id, new_book.clone());
            format!("Post book: {}", &new_book)
        } else {
            format!("Book id not found: {}", &new_book.id)
        }
    })
    .join()
    .unwrap()
    .into()
}

////
// HTML rendering helpers.
////

/// Render strings into an HTML table tag.
pub fn html_table_tag(table: Vec<Vec<String>>) -> String {
    format!("<table>\n{}</table>\n", html_table_tr_tags(table))
}

/// Render strings into HTML table tr tags.
pub fn html_table_tr_tags(rows: Vec<Vec<String>>) -> String {
    rows.iter()
        .map(|row| 
            format!("<tr>{}</tr>\n", html_table_td_tags(row))
        )
        .collect::<String>()
}

/// Render strings into HTML table td tags.
pub fn html_table_td_tags(cells: &Vec<String>) -> String {
    cells.iter().map(|cell| 
        format!("<td>{}</td>", cell)
    ).collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum_test::TestServer;

    #[tokio::test]
    async fn uptime() {
        let server = TestServer::new(app()).unwrap();
        let response_text_0 = server.get("/uptime").await.text();
        std::thread::sleep(std::time::Duration::from_secs(1));
        let response_text_1 = server.get("/uptime").await.text();
        assert!(response_text_0 < response_text_1, "{} < {}", response_text_0, response_text_1);
    }

    #[tokio::test]
    async fn count() {
        let server = TestServer::new(app()).unwrap();
            let response_text_0 = server.get("/count").await.text();
        let response_text_1 = server.get("/count").await.text();
        assert!(response_text_0 < response_text_1, "{} < {}", response_text_0, response_text_1);
    }

    #[tokio::test]
    async fn epoch() {
        let server = TestServer::new(app()).unwrap();
        let response_text_0 = server.get("/epoch").await.text();
        std::thread::sleep(std::time::Duration::from_secs(1));
        let response_text_1 = server.get("/epoch").await.text();
        assert!(response_text_0 < response_text_1, "{} < {}", response_text_0, response_text_1)
    }


}
