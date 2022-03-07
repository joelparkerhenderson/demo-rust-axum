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
```

Run:

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

use axum::{routing::get, Router};

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

Run:

```sh
cargo run
```

Browse <http://localhost:3000> and you should see "Hello, World!".


## Router and Handler

A route can call a function, which is called a handler.

Edit `main.rs` to add a handler function that returns text:

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

Run:

```sh
cargo run
```

Browse <http://localhost:3000> and you should see "Hello, World!".


## Add routes and handlers

Add routes:

```rust
let app = Router::new()
    .route("/", get(hello))
    .route("/foo", get(foo))
    .route("/bar", get(bar))
```

Add handlers:

```rust
async fn foo() -> String {
   "Foo!".to_string()
}

async fn bar() -> String {
   "Bar!".to_string()
}
```

Run:

```sh
cargo run
```

Browse <http://localhost:3000/foo> and you should see "Foo!".

Browse <http://localhost:3000/bar> and you should see "Bar!".


## Add HTTP verbs GET and POST

Axum uses HTTP verbs, including GET to fetch data, POST to submit data, etc.

Add routes for GET and POST:

```rust
let app = Router::new()
    .route("/", get(root))
    .route("/foo", get(get_foo).post(post_foo))
    .route("/bar", get(get_bar).post(post_bar));
```

Update handlers:

```rust
async fn get_foo() -> String {
   "Get Foo!".to_string()
}

async fn post_foo() -> String {
   "Post Foo!".to_string()
}

async fn get_bar() -> String {
   "Get Bar!".to_string()
}

async fn post_bar() -> String {
   "Post Bar!".to_string()
}
```

Run:

```sh
cargo run
```

Browse <http://localhost:3000/foo> and you should see "Get Foo!".

Browse <http://localhost:3000/bar> and you should see "Get Bar!".

To explicity try the GET verb and POST verb, one way is to use a command line
program such as `curl` like this:

```sh
$ curl -X GET 'http://localhost:3000/foo'
Get Foo!

$ curl -X POST 'http://localhost:3000/foo'
Post Foo!

$ curl -X GET 'http://localhost:3000/bar'
Get Bar!

$ curl -X POST 'http://localhost:3000/bar'
Post Bar!
```
