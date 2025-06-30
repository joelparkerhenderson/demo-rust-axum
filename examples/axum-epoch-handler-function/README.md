# Epoch handler function

An axum route can call an axum handler, which is an async function that returns
anything that axum can convert into a response.

Edit file `main.rs` router code to add this route:

```rust
let app = axum::Router::new()
    …
    .route("/epoch",
        get(epoch)
    )
    …
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
