## Respond with HTTP status code

Edit file `main.rs`.

Add a route:

```rust
let app = Router::new()
    …
    .route("/demo-http-status-code",
        get(demo_http_status_code)
    );
```

Add a handler:

```rust
/// axum handler for "GET /demo-http-status-code" which returns a 
/// HTTP status code, such as OK (200), and a visible string message.
pub async fn demo_http_status_code() -> (
    axum::http::StatusCode, String
) {
    (axum::http::StatusCode::OK, "Everything is OK".to_string())
}
```


### Try the demo…

Shell:

```sh
cargo run
```

Browse <http://localhost:3000/demo-http-status-code>

You should see "Everything is OK".
