# Axum extract path parameters

Add a route using path parameter syntax, such as ":id", in order to tell axum to
extract a path parameter and deserialize it into a variable named `id`.

Edit file `main.rs`.

Add a route:

```rust
let app = Router::new()
    …
    .route("/demo-path/:id",
        get(get_demo_path_id)
    );
```

Add a handler:

```rust
/// axum handler for "GET /demo-path/:id" which uses `axum::extract::Path`.
/// This extracts a path parameter then deserializes it as needed.
pub async fn get_demo_path_id(
    axum::extract::Path(id):
        axum::extract::Path<String>
) -> String {
    format!("Get demo path id: {:?}", id)
}
```


### Try the demo…

Shell:

```sh
cargo run
```

Shell:

```sh
curl 'http://localhost:3000/demo-path/1'
```

Ouput:

```sh
Get demo path id: 1
```
