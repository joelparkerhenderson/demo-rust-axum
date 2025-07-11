# Extract query parameters

Edit file `main.rs`.

Add code to use HashMap to deserialize query parameters into a key-value map:

```rust
use std::collections::HashMap;
```

Edit function `app`.

Add a route:

```rust
pub fn app() -> axum::Router {
    axum::Router::new()
        …
        .route("/demo-query",
            get(get_demo_query)
        )
}
```

Add a handler:

```rust
/// axum handler for "GET /items" which uses `axum::extract::Query`.
/// This extracts query parameters and creates a key-value pair map.
pub async fn get_items(
    axum::extract::Query(params):
        axum::extract::Query<HashMap<String, String>>
) -> String {
    format!("Get items with query params: {:?}", params)
}
```

Add a test:

```rust
#[tokio::test]
async fn test() {
    let server = TestServer::new(app()).unwrap();
    server.get("/demo-query?a=b").await.assert_text("Demo query params: {\"a\": \"b\"}");
}
```

## Try the demo

Shell:

```sh
cargo run
```

Shell:

```sh
curl 'http://localhost:3000/items?a=b'
```

Output:

```sh
Get items with query params: {"a": "b"}
```
