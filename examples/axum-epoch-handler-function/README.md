# Epoch handler function

An axum route can call an axum handler, which is an async function that returns
anything that axum can convert into a response.

Edit file `main.rs` function `app` and add this route:

```rust
pub fn app() -> axum::Router {
    axum::Router::new()
        …
        .route("/epoch",
            get(epoch)
        )
}
```

Add a handler that returns a Result of Ok(String) or Err(axum::http::StatusCode):

```rust
/// axum handler for "GET /epoch" which shows the current epoch time.
/// This shows how to write a handler that uses time and can error.
pub async fn epoch() -> Result<String, axum::http::StatusCode> {
    match std::time::SystemTime::now().duration_since(std::time::SystemTime::UNIX_EPOCH) {
        Ok(duration) => Ok(format!("{}", duration.as_secs())),
        Err(_) => Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
    }
}
```

Add a test:

```rust
#[tokio::test]
async fn test() {
    let server = TestServer::new(app()).unwrap();
    let response_text_0 = server.get("/epoch").await.text();
    std::thread::sleep(std::time::Duration::from_secs(1));
    let response_text_1 = server.get("/epoch").await.text();
    assert!(response_text_0 < response_text_1, "{} < {}", response_text_0, response_text_1)
}
```

## Try the demo

Shell:

```sh
cargo run
```

Browse <http://localhost:3000/epoch> or run:

```sh
curl 'http://localhost:3000/epoch'
```

You should see the epoch represented as seconds since January 1 1970 such as:

```txt
1750938523
```
