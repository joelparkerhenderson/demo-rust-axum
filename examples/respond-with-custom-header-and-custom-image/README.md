# Respond with custom header and custom image

Edit file `Cargo.toml`.

Add dependencies:

```rust
base64 = { version = "~0.22.1" } # Encode and decode base64 as bytes or utf8.
http = { version = "~1.3.1" } # Types for HTTP requests and responses.
```

Edit file `main.rs`.

Add a route:

```rust
let app = axum::Router::new()
    …
    .route("/demo.png",
        get(get_demo_png)
    )
```

Add a handler:

```rust
/// axum handler for "GET /demo.png" which responds with an image PNG.
/// This sets a header "image/png" then sends the decoded image data.
async fn get_demo_png() -> impl axum::response::IntoResponse {
    use base64::Engine;
    let png = concat!(
        "iVBORw0KGgoAAAANSUhEUgAAAAEAAAAB",
        "CAYAAAAfFcSJAAAADUlEQVR42mPk+89Q",
        "DwADvgGOSHzRgAAAAABJRU5ErkJggg=="
    );
    (
        axum::response::AppendHeaders([
            (axum::http::header::CONTENT_TYPE, "image/png"),
        ]),
        base64::engine::general_purpose::STANDARD.decode(png).unwrap(),
    )
}
```

## Try the demo

Shell:

```sh
cargo run
```

Browse <http://localhost:3000/demo.png>

You browser should download a one-pixel transparent PNG image.
