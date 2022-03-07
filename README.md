# Demo Rust Axum

Demonstration of:

* Rust: programming language

* Axum: web framework that focuses on ergonomics and modularity.

* Tower: library of modular and reusable components for building robust clients and servers.

* Tokio: event-driven, non-blocking I/O platform for writing asynchronous I/O backed applications. 

* Hyper: fast and correct HTTP library.

* Serde: serialization/deserialization framework.


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
async fn hello_handler() -> String {
   "Hello, World!".to_string()
}
```

Change the `main.rs` router to this:

```rust
let app = Router::new()
    .route("/", get(hello_handler));
```

Run:

```sh
cargo run
```

Browse <http://localhost:3000> and you should see "Hello, World!".
