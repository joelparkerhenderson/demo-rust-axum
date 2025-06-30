# Respond with JSON

Edit file `Cargo.toml` to add dependencies:

```toml
serde = { version = "~1.0.219", features = ["derive"] } # A serialization/deserialization framework.
serde_json = { version = "~1.0.140" } # Serde serialization/deserialization of JSON data.
```

Edit file `main.rs`.

Add a route to get the demo JSON:

```rust
let app = Router::new()
    …
    .route("/demo.json",
        get(get_demo_json)
    );
```

Add a handler:

```rust
/// axum handler for "GET /demo.json" which returns JSON data.
/// The `Json` type sets an HTTP header content-type `application/json`.
/// The `Json` type supports types that implement `serde::Deserialize`.
pub async fn get_demo_json() -> axum::extract::Json<serde_json::Value> {
    serde_json::json!({"a":"b"}).into()
}
```

## Try the demo

Shell:

```sh
cargo run
```

To request JSON with curl, set a custom HTTP header like this:

```sh
curl \
--header "Accept: application/json" \
--request GET 'http://localhost:3000/demo.json'
```

Output:

```sh
{"a":"b"}
```
