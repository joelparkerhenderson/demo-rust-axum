# Create a router fallback

To handle a request that fails to match anything in the axum router,
you can use the function `fallback`.

Edit file `main.rs`.

Add a `fallback` handler:

```rust
/// axum handler for any request that fails to match the router routes.
/// This implementation responds with HTTP status code NOT FOUND (404).
pub async fn fallback(
    uri: axum::http::Uri
) -> impl axum::response::IntoResponse {
    (axum::http::StatusCode::NOT_FOUND, uri.to_string())
}
```

Modify the `Router` to add the function `fallback` as the first choice:

```rust
let app = Router::new()
    .fallback(
        fallback
    )
    …
```

## Try the demo…

Shell:

```sh
cargo run
```

Browse <http://localhost:3000/foo> or run curl:

```sh
curl 'http://localhost:3000/foo'
```

Output:

```sh
/foo
```

In your shell, press CTRL-C to shut down.
