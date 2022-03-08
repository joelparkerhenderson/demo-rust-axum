# Demo Rust Axum

Demonstration of:

* [Rust](https://www.rust-lang.org): programming language

* [Axum](https://crates.io/crates/axum): web framework that focuses on ergonomics and modularity.

* [Tower](https://crates.io/crates/tower): library of modular and reusable components for building robust clients and servers.

* [Tokio](https://crates.io/crates/tokio): event-driven, non-blocking I/O platform for writing asynchronous I/O backed applications.

* [Hyper](https://crates.io/crates/hyper): fast and correct HTTP library.

* [Serde](https://crates.io/crates/serde): serialization/deserialization framework.


## Create a demo

Create a typical new Rust project:

```
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

<details>
<summary>Try it</summary>

Shell:

```sh
cargo run
```

</details>


## Hello, World!

To write a simple application that uses a single route,
such as a root route that responds with "Hello, World!",
then we create a new route, specifying the root path "/",
and specifying an asynchronous response that returns text.

Edit file `src/main.rs` like this:

```rust
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
        .route("/", get(|| async { "Hello, World!" }));

    // Run our application by using hyper and URL http://localhost:3000.
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
```

<details>
<summary>Try it</summary>

Shell:

```sh
cargo run
```

Browse <http://localhost:3000>

You should see "Hello, World!".

</details>


## Router and Handler

An Axum route can call an function, which is called an Axum handler. The handler
is async function returns something that can be converted into a response.

Edit `main.rs` to add a handler async function that returns text:

```rust
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
<summary>Try it</summary>

Shell:

```sh
cargo run
```

Browse <http://localhost:3000>

You should see "Hello, World!".

</details>


## Add fallback and handler

For a request that fails to match anything in the router, you can use the function `fallback`.

Use Axum types:

```
use axum::{
    …
    handler::Handler,
    http::StatusCode,
    http::Uri,
    response::IntoResponse,
};
```

Add the router fallback as the first choice:

```
let app = Router::new()
    .fallback(fallback.into_service()),
    .route("/", get(hello));
```

Add a fallback handler:

```
async fn fallback(uri: Uri) -> impl IntoResponse {
    (StatusCode::NOT_FOUND, format!("No route for {}", uri))
}
```

<details>
<summary>Try it</summary>

Shell:

```sh
cargo run
```

Browse <http://localhost:3000/whatever>

You should see "No route for /whatever".

</details>


## Respond with an HTTP status code

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
    .route("/demo-ok", get(demo_status));
```

Add a handler:

```rust
async fn demo_status() -> (StatusCode, String) {
    (StatusCode::OK, "Everything is OK".to_string())
}
```

<details>
<summary>Try it</summary>

Shell:

```sh
cargo run
```

Browse <http://localhost:3000/demo-status>

You should see "Everything is OK".

</details>


## Respond with the current URI

Add a route:

```rust
let app = Router::new()
    …
    .route("/demo-uri", get(demo_uri));
```

Add a handler:

```rust
async fn demo_uri(uri: Uri) -> String {
    format!("The URI is: {:?}", uri)
}
```

<details>
<summary>Try it</summary>

Shell:

```sh
cargo run
```

Browse <http://localhost:3000/demo-uri>

You should see "The URI is: /demo-uri!".

</details>


## Add HTTP verbs for GET and POST

Axum uses HTTP verbs, including GET to fetch data, POST to submit data, etc.

Add routes for GET and POST:

```rust
let app = Router::new()
    …
    .route("/item", get(get_item).post(post_item))
```

Add handlers:

```rust
async fn get_item() -> String {
   "GET item".to_string()
}

async fn post_item() -> String {
   "POST item".to_string()
}
```

<details>
<summary>Try it</summary>

Shell:

```sh
cargo run
```

To make a request using an explicit GET verb or POST verb,
one way is to use a command line program such as `curl` like this:

Shell:

```sh
curl --request GET 'http://localhost:3000/item'
```

Output:

```sh
GET item
```

Shell:

```sh
curl --request POST 'http://localhost:3000/item'
```

Output:

```sh
GET item
```

</details>


## Get HTML content

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
<summary>Try it</summary>

Shell:

```sh
cargo run
```

Browse <http://localhost:3000/demo.html>

You should see HTML with headline text "Hello".

</details>


## Get JSON data

Use Serde JSON in order to format JSON data using structs:

```
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
async fn get_demo_json() -> Json<Value> {
    Json(json!({"a":"b"}))
}
```

<details>
<summary>Try it</summary>

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


## Extract JSON data

Add code to use `JSON`:

```rust
use axum::{
    …
    extract::Json,
};
```

Add a route:

```rust
let app = Router::new()
    …
    .route("/demo-json", get(get_demo_json));
```

Add a handler:

```rust
async fn get_demo_json(Json(payload): Json<serde_json::Value>) -> String{
    format!("Get demo JSON payload: {:?}", payload)
}
```

<details>
<summary>Try it</summary>

Shell:

```sh
cargo run
```

To use JSON with newer versions of curl:

```sh
curl \
--request GET 'http://localhost:3000/demo-json' \
--json '{"a":"b"}'
```

Output:

```sh
Get demo JSON payload: Object({"a": String("b")})
```

To use JSON with older versions of curl:

```sh
curl \
--request GET 'http://localhost:3000/demo-json' \
--header "Content-Type: application/json" \
--data '{"a":"b"}'
```

Output:

```sh
Get demo JSON payload: Object({"a": String("b")})
```

</details>


## Extract query parameters

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
    .route("/item", get(get_item));
```

Add a handler:

```rust
async fn get_item(Query(params): Query<HashMap<String, String>>) -> String {
    format!("Get item query params: {:?}", params)
}
```

<details>
<summary>Try it</summary>

Shell:

```sh
cargo run
```

Shell:

```sh
curl --request GET 'http://localhost:3000/item?a=b'
```

Output:

```
Get item query params: {"a": "b"}
```

</details>


## Extract path parameters

Add code to use `Path`:

```rust
use axum::{
    …
    extract::Query,
};
```

Add a route using path parameter syntax, such as ":id", in order to tell Axum to
extract a path parameter and deserialize it into a variable named `id`:

```rust
let app = Router::new()
    …
    .route("/item/:id", get(get_item_id));
```

Add a handler:

```rust
async fn get_item_id(Path(id): Path<u32>) {
    format!("Get item by id: {:?}", id).to_string()
}
```

<details>
<summary>Try it</summary>

Shell:

```sh
cargo run
```

Shell:

```sh
curl --request GET 'http://localhost:3000/item/1'
```

Ouput:

```sh
Get item by id: 1
```

</details>


## Return a struct by id

Suppose we want our app to be a bookstore.

Create a book struct:

```rust
struct Book {
    title: String,
    author: String,
}
```

Create a books data lookup:

```rust
async fn books() -> HashMap<u32, Book> {
    let mut books: HashMap<u32, Book> = HashMap::new();
    books.insert(1, Book { title: "Antigone".into(), author: "Sophocles".into()});
    books.insert(2, Book { title: "Beloved".into(), author: "Toni Morrison".into()});
    books.insert(3, Book { title: "Candide".into(), author: "Voltaire".into()});
    books
}
```

Add a route:

```rust
let app = Router::new()
    …
    .route("/book/:id", get(get_book_by_id));
```

Add a handler:

```rust
async fn get_book_by_id(Path(id): Path<u32>) {
    format!("Get item by id: {:?}", id).to_string()
}
```

<details>
<summary>Try it</summary>

Shell:

```sh
cargo run
```

Shell:

```sh
curl --request GET 'http://localhost:3000/book/1'
```

Output:

```sh
<h1>Book 1</h1> The book is Antigone by Sophocles"
```

</details>


## Bonus: Tracing subscriber

Edit file `Cargo.toml` to add crates:

```toml
tracing = "*"
tracing-subscriber = { version = "*", features = ["env-filter"] }
```

Edit file `main.rs` to use tracing:

```rust
use tracing_subscriber::{
    layer::SubscriberExt,
    util::SubscriberInitExt,
};
```

Add a tracing subscriber:

```rust
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();
    …
```

<details>
<summary>Try it</summary>

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


## Bonus: SocketAddr

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
