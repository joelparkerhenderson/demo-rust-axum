# Demo of Rust and Axum web framework

Demonstration of:

* [Rust](https://www.rust-lang.org): programming language that focuses on reliability and stability.

* [Axum](https://crates.io/crates/axum): web framework that focuses on ergonomics and modularity.

* [Tower](https://crates.io/crates/tower): library of modular and reusable components for building robust clients and servers.

* [Serde](https://crates.io/crates/serde): serialization/deserialization framework.

<!--

* [Tokio](https://crates.io/crates/tokio): event-driven, non-blocking I/O platform for writing asynchronous I/O backed applications.

* [Hyper](https://crates.io/crates/hyper): fast and correct HTTP library.

-->

Thanks to all the above projects and their authors. Donate to them if you can.

Does this demo help your work? Donate here if you can via GitHub sponsors.

Have an idea, suggestion, or feedback? Let us know via GitHub issues.


## 1. Hello, World!

Create a typical new Rust project:

```sh
cargo new demo_rust_axum
cd demo_rust_axum
```

Edit file `Cargo.toml` like this:

```toml
[package]
name = "demo-rust-axum"
version = "0.1.0"
edition = "2021"

[dependencies]
axum = "*"  # Web framework that focuses on ergonomics and modularity.
hyper = "*"  # A fast and correct HTTP library.
tokio = { version = "*", features = ["full"] }  # Event-driven, non-blocking I/O platform.
tower = "*"  # Components for building robust clients and servers.
serde = { version = "*", features = ["derive"] }  # Serialization/deserialization framework.
serde_json = "*"  # Serialization/deserialize of JSON data.
```

Edit file `src/main.rs` like this:

```rust
// Axum web framework.
extern crate axum;

// Tokio provides an event-driven, non-blocking I/O platform for writing
// asynchronous I/O backed applications; Axum leverages Tokio throughout.
extern crate tokio;

// Use Axum capabities.
use axum::{
    routing::get,
    Router,
};

#[tokio::main]
async fn main() {
     // Build our application by creating our router.
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }));

    // Run our application by using hyper and URL http://localhost:3000.
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
```

<details>
<summary>Interactive</summary>
<p><b>Try the demo…</b></p>

Shell:

```sh
cargo run
```

Browse <http://localhost:3000>

You should see "Hello, World!".

</details>


## 2. Create a new route and handler function

An Axum route can call an function, which is called an Axum handler. The handler
is async function returns something that can be converted into a response.

Edit `main.rs` to add a handler async function that returns text:

```rust
// Axum handler for "GET /" which returns a string, which causes Axum to
// immediately respond with a `200 OK` response, along with the plain text.
async fn hello() -> String {
   "Hello, World!".to_string()
}
```

Change the `main.rs` router to this:

```rust
let app = Router::new()
    .route("/", get(hello));
```

<details>
<summary>Interactive</summary>
<p><b>Try the demo…</b></p>

Shell:

```sh
cargo run
```

Browse <http://localhost:3000>

You should see "Hello, World!".

</details>


## 3. Create a router fallback response "not found"

For a request that fails to match anything in the router, you can use the function `fallback`.

Use Axum types:

```rust
use axum::{
    …
    handler::Handler,
    http::StatusCode,
    http::Uri,
    response::IntoResponse,
};
```

Add the router fallback as the first choice:

```rust
let app = Router::new()
    .fallback(fallback.into_service()),
    .route("/", get(hello));
```

Add a fallback handler:

```rust
// Axum handler for any request that fails to match the router routes.
// This implementation returns a HTTP status code 404 Not Found response.
async fn fallback(uri: Uri) -> impl IntoResponse {
    (StatusCode::NOT_FOUND, format!("No route for {}", uri))
}
```

<details>
<summary>Interactive</summary>
<p><b>Try the demo…</b></p>

Shell:

```sh
cargo run
```

Browse <http://localhost:3000/whatever>

You should see "No route for /whatever".

</details>


## 4. Create a response with HTTP status code OK

Add code to use `StatusCode`:

```rust
use axum::{
    …
    http::StatusCode,
};
```

Add a route:

```rust
let app = Router::new()
    …
    .route("/demo-status", get(demo_status));
```

Add a handler:

```rust
// Axum handler that implements `IntoResponse`, which allows us to return any
// HTTP status code (such as status code 200 OK) and any description string.
async fn demo_status() -> (StatusCode, String) {
    (StatusCode::OK, "Everything is OK".to_string())
}
```

<details>
<summary>Interactive</summary>
<p><b>Try the demo…</b></p>

Shell:

```sh
cargo run
```

Browse <http://localhost:3000/demo-status>

You should see "Everything is OK".

</details>


## 5. Create a response that echos the URI

Add a route:

```rust
let app = Router::new()
    …
    .route("/demo-uri", get(demo_uri));
```

Add a handler:

```rust
// Axum handler for "GET /demo.html" which shows to return HTML text.
// The `Html` type sets an HTTP header content-type of `text/html`.
async fn demo_uri(uri: Uri) -> String {
    format!("The URI is: {:?}", uri)
}
```

<details>
<summary>Interactive</summary>
<p><b>Try the demo…</b></p>

Shell:

```sh
cargo run
```

Browse <http://localhost:3000/demo-uri>

You should see "The URI is: /demo-uri!".

</details>


## 6. Create routes and handlers for GET, POST, DELETE.

Axum uses HTTP verbs, including GET to fetch data, POST to submit data, DELETE to destroy data, etc.

Add routes for GET and POST and DELETE:

```rust
let app = Router::new()
    …
    .route("/foo", get(get_foo).post(post_foo).delete(delete_foo))
```

Add handlers:

```rust
async fn get_foo() -> String {
   "GET foo".to_string()
}

async fn post_foo() -> String {
   "POST foo".to_string()
}

async fn delete_foo() -> String {
   "DELETE foo".to_string()
}
```

<details>
<summary>Interactive</summary>
<p><b>Try the demo…</b></p>

Shell:

```sh
cargo run
```

To make a request using an explicit request of GET or POST or DELETE,
one way is to use a command line program such as `curl` like this:

Shell:

```sh
curl --request GET 'http://localhost:3000/foo'
```

Output:

```sh
GET foo
```

Shell:

```sh
curl --request POST 'http://localhost:3000/foo'
```

Output:

```sh
POST foo
```

Shell:

```sh
curl --request DELETE 'http://localhost:3000/foo'
```

Output:

```sh
DELETE foo
```

The command `curl` uses GET by default i.e. these are equivalent:

```sh
curl 'http://localhost:3000/foo'

curl --request GET 'http://localhost:3000/foo'
```

</details>


## 7. Create a reponse with HTML text

Add code to use `Html`:

```rust
use axum::{
    …
    response::Html,
};
```

Add route:

```rust
let app = Router::new()
    …
    .route("/demo.html", get(get_demo_html));
```

Add handler:

```rust
async fn get_demo_html() -> Html<&'static str> {
    Html("<h1>Hello</h1>")
}
```

<details>
<summary>Interactive</summary>
<p><b>Try the demo…</b></p>

Shell:

```sh
cargo run
```

Browse <http://localhost:3000/demo.html>

You should see HTML with headline text "Hello".

</details>


## 8. Create a route that gets JSON data

Use Serde JSON in order to format JSON data:

```rust
// Use Serde JSON to serialize/deserialize JSON, such as the request body.
// Axum creates JSON payloads or extracts them by using `axum::extract::Json`.
// For the implementation, see functions `get_demo_json` and `post_demo_json`.
use serde_json::{json, Value};
```

Add a route:

```rust
let app = Router::new()
    …
    .route("/demo-json", get(get_demo_json));
```

Add a handler:

```rust
// Axum handler for "GET /demo.json" which shows how to return JSON data.
// The `Json` type sets an HTTP header content-type of `application/json`.
// The `Json` type works with any type that implements `serde::Serialize`.
async fn get_demo_json() -> Json<Value> {
    Json(json!({"a":"b"}))
}
```

<details>
<summary>Interactive</summary>
<p><b>Try the demo…</b></p>

Shell:

```sh
cargo run
```

To JSON with curl, set a custom HTTP header like this:

```sh
curl \
--header "Content-Type: application/json" \
--request GET 'http://localhost:3000/demo-json'
```

Output:

```sh
{"a":"b"}
```

</details>


## 9. Create a route that extracts its JSON payload

Add code to use `JSON`:

```rust
use axum::{
    …
    extract::Json,
};
```

Append this route with post:

```rust
let app = Router::new()
    …
    .route("/demo.json", get(get_demo_json).post(post_demo_json))
```

Add a handler:

```rust
// Axum handler for "POST /demo-json" which shows how to use `aumx::extract::Json`.
// This buffers the request body then deserializes it into a `serde_json::Value`.
// The Axum `Json` type supports any type that implements `serde::Deserialize`.
async fn post_demo_json(Json(payload): Json<serde_json::Value>) -> String{
    format!("Get demo JSON payload: {:?}", payload)
}
```

<details>
<summary>Interactive</summary>
<p><b>Try the demo…</b></p>

Shell:

```sh
cargo run
```

To use JSON with newer versions of curl:

```sh
curl \
--request POST 'http://localhost:3000/demo-json' \
--json '{"a":"b"}'
```

Output:

```sh
Get demo JSON payload: Object({"a": String("b")})
```

To use JSON with older versions of curl:

```sh
curl \
--request POST 'http://localhost:3000/demo-json' \
--header "Content-Type: application/json" \
--data '{"a":"b"}'
```

Output:

```sh
Get demo JSON payload: Object({"a": String("b")})
```

</details>


## 10. Create a route that extracts query parameters

An Axum "extractor" is how you pick apart the incoming request in order to get
any parts that your handler needs.

Add code to use `Query`:

```rust
use axum::{
    …
    extract::Query,
};
```

Use HashMap to deserialize query parameters into a key-value map:

```rust
use std::collections::HashMap;
```

Add a route:

```rust
let app = Router::new()
    …
    .route("/items", get(get_items));
```

Add a handler:

```rust
// Axum handler for "GET /item" which shows how to use `axum::extrac::Query`.
// This extracts query parameters then deserializes them into a key-value map.
async fn get_items(Query(params): Query<HashMap<String, String>>) -> String {
    format!("Get items with query params: {:?}", params)
}
```

<details>
<summary>Interactive</summary>
<p><b>Try the demo…</b></p>

Shell:

```sh
cargo run
```

Shell:

```sh
curl 'http://localhost:3000/items?a=b'
```

Output:

```sh
Get items with query params: {"a": "b"}
```

</details>


## 11. Create a route that extracts path parameters

Add code to use `Path`:

```rust
use axum::{
    …
    extract::Path,
};
```

Add a route using path parameter syntax, such as ":id", in order to tell Axum to
extract a path parameter and deserialize it into a variable named `id`:

```rust
let app = Router::new()
    …
    .route("/items/:id", get(get_items_id));
```

Add a handler:

```rust
// Axum handler for "GET /item/:id" which shows how to use `axum::extract::Path`.
// This extracts a path parameter then deserializes it into an integer.
async fn get_items_id(Path(id): Path<u32>) {
    format!("Get items with id: {:?}", id).to_string()
}
```

<details>
<summary>Interactive</summary>
<p><b>Try the demo…</b></p>

Shell:

```sh
cargo run
```

Shell:

```sh
curl 'http://localhost:3000/items/1'
```

Ouput:

```sh
Get items with id: 1
```

</details>


## 11. Create a data store of books

Suppose we want our app to be a data store of books.

Create a book struct:

```rust
// Demo book structure that we can debug, clone, hash, and compare.
#[derive(Debug, Clone, Hash, ParialEq, Eq)]
struct Book {
    id: u32,
    title: String,
    author: String,
}
```

For a production app, we could implement the data by using a database.

For this demo app, we will implement the data by using a global variable `BOOKS`.

Edit file `Cargo.toml` to add the crate `once_cell` for global variables:

```toml
once_cell = "*"
```

Edit file `main.rs` to add:

```rust
// Use once_cell for creating a global variable e.g. our BOOKS data.
use once_cell::sync::Lazy;

// Use Mutex for thread-safe access to a variable e.g. our BOOKS data.
use std::sync::Mutex;

// Use HashSet for a collection of items e.g. our BOOKS data.
use std::collections::HashSet;

// Create our BOOKS data by using a global variable with thread-safe access.
static BOOKS: Lazy<Mutex<HashSet<Book>>> = Lazy::new(|| Mutex::new(HashSet::new()));
```

Add a function to initialize the BOOKS global variable with demo data:

```rust
// Initialize the BOOKS global variable by inserting demo data.
async fn init_books() {
    for book in vec![
        Book { id: 1, title: "Antigone".into(), author: "Sophocles".into()},
        Book { id: 2, title: "Beloved".into(), author: "Toni Morrison".into()},
        Book { id: 3, title: "Candide".into(), author: "Voltaire".into()},
    ] {
        BOOKS.lock().unwrap().insert(book);
    }
}
```

Call the function at the start of `main()`:

```rust
async fn main() {
    // Initialize our demo data for the examples about books.
    init_books().await;
    …
}
```

<details>
<summary>Interactive</summary>
<p><b>Try the demo…</b></p>

Shell:

```sh
cargo run
```

The app should launch normally.

</details>



## 12. Create a route to get all books

Add a route:

```rust
let app = Router::new()
    …
    .route("/books", get(get_books));
```

Add a handler:

```rust
// Axum handler for "GET /books" which returns a resource index HTML page.
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
```

<details>
<summary>Interactive</summary>
<p><b>Try the demo…</b></p>

Shell:

```sh
cargo run
```

Shell:

```sh
curl 'http://localhost:3000/books'
```

Output:

```sh
<p>Antigone by Sophocles</p>
<p>Beloved by Toni Morrison</p>
<p>Candide by Voltaire</p>
```

</details>


## 13. Create a route to get one book id

Add a route:

```rust
let app = Router::new()
    …
    .route("/books/:id", get(get_books));
```

Add a handler:

```rust
// Axum handler for "GET /books/:id" which returns one resource HTML page.
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
```

<details>
<summary>Interactive</summary>
<p><b>Try the demo…</b></p>

Shell:

```sh
cargo run
```

Shell:

```sh
curl 'http://localhost:3000/books/1'
```

Output:

```sh
<p>Antigone by Sophocles</p>
```

Shell:

```sh
curl 'http://localhost:3000/books/0'
```

Output:

```sh
<p>Not found</p>
```

</details>


## 14. Bonus: Add a Tower tracing subscriber

Edit file `Cargo.toml` to add crates:

```toml
tracing = "*"
tracing-subscriber = { version = "*", features = ["env-filter"] }
```

Edit file `main.rs` to use tracing:

```rust
// Use tracing crates for application-level tracing output.
use tracing_subscriber::{
    layer::SubscriberExt,
    util::SubscriberInitExt,
};
```

Add a tracing subscriber:

```rust
async fn main() {
    // Start tracing.
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();
    …
```

<details>
<summary>Interactive</summary>
<p><b>Try the demo…</b></p>

Shell:

```sh
cargo run
```

You should see console output that shows tracing initialization such as:

```
2022-03-08T00:13:54.483877Z
    TRACE mio::poll:
    registering event source with poller:
    token=Token(1),
    interests=READABLE | WRITABLE
```

</details>


## 15. Bonus: Refactor to use a host, port, and socket address

To bind the server, our demo code uses a socket address string:

```rust
axum::Server::bind(&"0.0.0.0:3000".parse().unwrap()) …
```

If you prefer create a socket address step by step, then you can do this:

```rust
use std::net::SocketAddr;

async fn main() {
    …
    let host = [127, 0, 0, 1];
    let port = 3000;
    let addr = SocketAddr::from((host, port));
    axum::Server::bind(&addr) …
```

## 16. Conclusion: What you learned and what's next

You learned how to:

* Create a project using Rust and the Axum web framework.

* Create Axum router routes and their handler functions.

* Create responses with HTTP status code OK and HTML text.

* Create functionality for HTTP GET and HTTP POST.

* Use Axum extractors for query parameters and path parameters.

* Create a data store and access it using RESTful routes.

What's next:

* [The Rust book](https://doc.rust-lang.org/stable/book/) is an excellent thorough starting point.

* [The Axum crate](https://crates.io/crates/axum) has dozens of examples you can try.

* What suggestions and feedback do you have for us?
