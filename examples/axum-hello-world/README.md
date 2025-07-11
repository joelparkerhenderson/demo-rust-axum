# Hello, World

Create a typical new Rust project:

```sh
cargo new demo-rust-axum
cd demo-rust-axum
```

Edit file `Cargo.toml`.

Use this kind of package and these dependencies:

```toml
[package]
name = "demo-rust-axum-examples-axum-hello-world"
version = "1.0.0"
edition = "2024"

[dependencies]
axum = { version = "~0.8.4" } # Web framework that focuses on ergonomics and modularity.
tokio = { version = "~1.45.1", features = ["full"] } # Event-driven, non-blocking I/O platform.
```

Edit file `src/main.rs`.

```rust
#[tokio::main]
async fn main() {
    // Create our app with one route that prints "Hello, World!"
    let app = axum::Router::new()
        .route("/",
            axum::routing::get(|| async { "Hello, World!" })
        );

    // Run our application as a hyper server on http://localhost:3000.
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
```

## Try the demo

Shell:

```sh
cargo run
```

Browse <http://localhost:3000> or run:

```sh
curl 'http://localhost:3000'
```

Output:

```sh
Hello, World!
```

In your shell, press CTRL-C to shut down.
