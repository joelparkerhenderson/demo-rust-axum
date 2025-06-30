# Respond with request URI

Edit file `main.rs`.

Add a route:

```rust
let app = axum::Router::new()
    …
    .route("/demo-uri",
        get(demo_uri)
    );
```

Add a handler:

```rust
/// axum handler for "GET /demo-uri" which shows the request's own URI.
/// This shows how to write a handler that receives the URI.
pub async fn demo_uri(uri: axum::http::Uri) -> String {
    format!("The URI is: {:?}", uri)
}
```

## Try the demo

Shell:

```sh
cargo run
```

Browse <http://localhost:3000/demo-uri>

You should see "The URI is: /demo-uri!".
