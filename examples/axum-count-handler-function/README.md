# Count handler function

An axum route can call an axum handler, which is an async function that returns
anything that axum can convert into a response.

Edit file `main.rs`.

Add this anywhere before the function `main`:

```rust
/// Create the atomic variable COUNT so the program can track its own count.
pub static COUNT: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
```

Edit file `main.rs` function `app` and add this route:

```rust
pub fn app() -> axum::Router {
    axum::Router::new()
        …
        .route("/count",
            get(count)
        )
```

Add a handler that does an atomic increment of the count:

```rust
/// axum handler for "GET /count" which shows the program's count duration.
/// This shows how to write a handler that uses a global static lazy value.
pub async fn count() -> String {
    COUNT.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    format!("{}", COUNT.load(std::sync::atomic::Ordering::SeqCst))
}
```

Add a test:

```rust
#[tokio::test]
async fn test() {
    let server = TestServer::new(app()).unwrap();
    let response_text_0 = server.get("/count").await.text();
    let response_text_1 = server.get("/count").await.text();
    assert!(response_text_0 < response_text_1, "{} < {}", response_text_0, response_text_1);
}
```

## Try the demo

Shell:

```sh
cargo run
```

Browse <http://localhost:3000/count> or run:

```sh
curl 'http://localhost:3000/count'
```

You should see the hit count.

```txt
1
```

Reload and you should see the hit count increment:

You should see the hit count.

```txt
2
```
