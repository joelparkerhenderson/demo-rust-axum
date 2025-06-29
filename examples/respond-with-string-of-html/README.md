# Respond with string of HTML

Edit file `main.rs`.

Add a route:

```rust
let app = axum::Router::new()
    …
    .route("/string.html",
        get(get_string_html)
    )
    …
```

Add a handler:

```rust
/// axum handler for "GET /string.html" which responds with a string.
/// The `Html` type sets an HTTP header content-type of `text/html`.
pub async fn get_string_html() -> axum::response::Html<&'static str> {
    "<html><body><h1>Headline</h1><p>Paragraph</b></body></html>".into()
}
```
