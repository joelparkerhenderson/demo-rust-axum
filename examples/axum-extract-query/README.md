# Axum extract query parameters

Edit file `main.rs`.

Add code to use HashMap to deserialize query parameters into a key-value map:

```rust
use std::collections::HashMap;
```

Add a route:

```rust
let app = Router::new()
    …
    .route("/items",
        get(get_items)
    );
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


### Try the demo…

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


<div style="page-break-before:always;"></div>
