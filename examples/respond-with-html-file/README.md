# Respond with HTML file

Create file `file.html`.

Add this:

```html
<html>
    <body>
        <h1>Respond with HTML file</h1>
        <p>This is our example HTML file</p>
    </body>
</html>
```

Edit file `main.rs`.

Add route:

```rust
let app = axum::Router::new()
    …
    .route("/file.html",
        get(file_html)
    )
```

Add handler:

```rust
/// axum handler that responds with typical HTML coming from a file.
/// This uses the Rust macro `std::include_str` to include a UTF-8 file
/// path, relative to `main.rs`, as a `&'static str` at compile time.
async fn file_html() -> axum::response::Html<&'static str> {
    include_str!("file.html").into()
}
```

## Try the demo

Shell:

```sh
cargo run
```

Browse <http://localhost:3000/file.html> or run:

```sh
curl "http://localhost:3000/file.html"
```

You should see the headline "Respond with HTML file" and paragraph "This is our example HTML file".
