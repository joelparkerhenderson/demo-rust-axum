# axum test server

Create a typical new Rust project:

```sh
cargo new axum-test-server
cd axum-test-server
```

Edit file `Cargo.toml`.

Use this kind of package and these dependencies:

```toml
[package]
name = "axum-test-server"
version = "1.0.0"
edition = "2024"

[dependencies]
axum = { version = "~0.8.4" } # Web framework that focuses on ergonomics and modularity.
tokio = { version = "~1.45.1", features = ["full"] } # Event-driven, non-blocking I/O platform.

[dev-dependencies]
axum-test = { version = "17.3.0" } # Library for writing tests for web servers written using Axum.
```

Edit file `src/main.rs`.

```rust
#[tokio::main]
pub async fn main() {
    // Run our application as a hyper server on http://localhost:3000.
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app()).await.unwrap();
}

pub fn app() -> axum::Router {
    axum::Router::new()
    .route("/",
        axum::routing::get(|| async { "Hello, World!" })
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum_test::TestServer;

    #[tokio::test]
    async fn response_text() {
        let app: axum::Router = app();
        let server = TestServer::new(app).unwrap();
        let response_text = server.get("/").await.text();
        assert_eq!(response_text, "Hello, World!");
    }
}
```

## Try the demo test

Shell:

```sh
cargo test
```

Output:

```stdout
running 1 test
test tests::response_text ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```
