# Extract JSON parameters

The axum extractor for JSON deserializes a request body into any type that
implements `serde::Deserialize`. If the extractor is unable to parse the request
body, or if the request is missing the header `Content-Type: application/json`,
then the extractor returns HTTP `BAD_REQUEST` (404).

Edit file `Cargo.toml` to add dependencies:

```toml
serde = { version = "~1.0.219", features = ["derive"] } # A serialization/deserialization framework.
serde_json = { version = "~1.0.140" } # Serde serialization/deserialization of JSON data.
```

Edit file `main.rs`.

Add a route to put the demo JSON:

```rust
let app = Router::new()
    …
    .route("/demo.json",
        .put(put_demo_json)
    )
```

Add a handler:

```rust
/// axum handler for "PUT /demo.json" which uses `aumx::extract::Json`.
/// This buffers the request body then deserializes it using serde.
/// The `Json` type supports types that implement `serde::Deserialize`.
pub async fn put_demo_json(
    axum::extract::Json(data): axum::extract::Json<serde_json::Value>
) -> String{
    format!("Put demo JSON data: {:?}", data)
}
```

## Try the demo

Shell:

```sh
cargo run
```

Send the JSON:

```sh
curl \
--request PUT 'http://localhost:3000/demo.json' \
--header "Content-Type: application/json" \
--data '{"a":"b"}'
```

Output:

```sh
Put demo JSON data: Object({"a": String("b")})
```
