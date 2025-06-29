# Status handler function

Edit file `main.rs`.

Add a route:

```rust
let app = axum::Router::new()
    …
    .route("/status",
        get(status)
    );
```

Add a handler:

```rust
/// axum handler for "GET /status" which returns the HTTP status
/// code OK (200) along with a user-visible string message.
pub async fn status() -> (axum::http::StatusCode, String) {
    (axum::http::StatusCode::OK, "OK".to_string())
}
```

## Try the demo

Shell:

```sh
cargo run
```

Browse <http://localhost:3000/status>

You should see "OK".

