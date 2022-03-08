# Demo Rust Axum

Demonstration of:

* [Rust](https://www.rust-lang.org): programming language

* [Axum](https://crates.io/crates/axum): web framework that focuses on ergonomics and modularity.

* [Tower](https://crates.io/crates/tower): library of modular and reusable components for building robust clients and servers.

* [Tokio](https://crates.io/crates/tokio): event-driven, non-blocking I/O platform for writing asynchronous I/O backed applications.

* [Hyper](https://crates.io/crates/hyper): fast and correct HTTP library.

* [Serde](https://crates.io/crates/serde): serialization/deserialization framework.


## Cargo.toml

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

Try it:

```sh
cargo build
```


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

### Try it

```sh
cargo run
```

Browse <http://localhost:3000>

You should see "Hello, World!".


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

### Try it

```sh
cargo run
```

Browse <http://localhost:3000>

You should see "Hello, World!".


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

### Try it

```sh
cargo run
```

Browse <http://localhost:3000/whatever>

You should see "No route for /whatever".



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

### Try it

```sh
cargo run
```

Browse <http://localhost:3000/demo-status>

You should see "Everything is OK".



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

### Try it

```sh
cargo run
```

Browse <http://localhost:3000/demo-uri>

You should see "The URI is: /demo-uri!".


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

### Try it

```sh
cargo run
```

To make a request using an explicit GET verb or POST verb,
one way is to use a command line program such as `curl` like this:

```sh
$ curl --request GET 'http://localhost:3000/item'
GET item

$ curl --request POST 'http://localhost:3000/item'
POST item
```


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

### Try it

```sh
cargo run
```

Browse <http://localhost:3000/demo.html>

You should see HTML with headline text "Hello".


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

### Try it

```sh
cargo run
```

To JSON with curl, set a custom HTTP header like this:

```sh
$ curl \
    --header "Content-Type: application/json" \
    --request GET 'http://localhost:3000/demo-json'
{"a":"b"}
```


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

### Try it

```sh
cargo run
```

To use JSON with newer versions of curl:

```sh
$ curl \
    --request GET 'http://localhost:3000/demo-json' \
    --json '{"a":"b"}'
Get demo JSON payload: Object({"a": String("b")})
```

To use JSON with older versions of curl:

```sh
$ curl \
    --request GET 'http://localhost:3000/demo-json' \
    --header "Content-Type: application/json" \
    --data '{"a":"b"}'
Get demo JSON payload: Object({"a": String("b")})
```


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

### Try it

```sh
cargo run
```

```sh
$ curl --request GET 'http://localhost:3000/item?a=b'
Get item query params: {"a": "b"}
```


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
    .route("/item/:id", get(get_item_by_id));
```

Add a handler:

```rust
async fn get_item_by_id(Path(id): Path<u32>) {
    format!("Get item by id: {:?}", id).to_string()
}
```

### Try it

```sh
cargo run
```

```sh
$ curl --request GET 'http://localhost:3000/item/1'
Get item by id: 1
```


## Tracing subscriber

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

### Try it

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
