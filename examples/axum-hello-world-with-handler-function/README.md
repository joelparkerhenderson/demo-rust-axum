# Hello, World! with handler function

We can improve our "Hello, World!" by refactoring the app to use an axum handler function.

An axum handler function is an async function that returns anything that axum
can convert into a response. An axum route can call an an axum handler function.

Before the refactor, the code is:

```rust
/// Create our application with one route that prints "Hello, World!"
pub fn app() -> axum::Router {
    axum::Router::new()
        .route("/", get(|| async { "Hello, World!" }))
}
```

Edit file `main.rs` and add an axum handler function like this:

```rust
/// Create our application with one route that prints "Hello, World!"
pub fn app() -> axum::Router {
    axum::Router::new()
        .route("/", get(hello))
}

/// axum handler function which returns a string and causes axum to
/// immediately respond with status code `200 OK` and the string.
pub async fn hello() -> String {
   "Hello, World!".into()
}
```
