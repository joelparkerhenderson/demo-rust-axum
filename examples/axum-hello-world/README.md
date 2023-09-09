# Hello World

Example Axum Server that returns "Hello, World":

```rust
#[tokio::main]
async fn main() {
    // Build our application with a single route.
    let app = axum::Router::new().route("/",
        axum::routing::get(|| async { "Hello, World!" }));

    // Run our application as a hyper server on http://localhost:3000.
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
```


### Try the demo…

Shell:

```sh
cargo run
```

Send the JSON:

```sh
curl 'http://localhost:3000'
```

Output:

```sh
Hello, World!
```
