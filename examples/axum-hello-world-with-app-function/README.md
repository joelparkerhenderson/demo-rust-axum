# Hello, World! with app function

We can improve our "Hello, World!" by refactoring the app into its own function.

Before the refactor, the code is:

```rust
#[tokio::main]
async fn main() {
    // Create our app with one route that prints "Hello, World!"
    let app = axum::Router::new()
        .route("/", axum::routing::get(|| async { "Hello, World!" }));

    // Run our application as a hyper server on http://localhost:3000.
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
```

Our demos will often use the axum routing function, so add code to use it:

```rust
/// Use axum routing.
use axum::routing::get;
```

Refactor the function `main` to move the app out:

```rust
/// Run our app using a hyper server on http://localhost:3000.
#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app()).await.unwrap();
}
```

Create the function `app`:

```rust
/// Create our application with one route that prints "Hello, World!"
pub fn app() -> axum::Router {
    axum::Router::new()
        .route("/", get(|| async { "Hello, World!" }))
}
```

After the refactor, the code is:

```rust
/// Use axum routing.
use axum::routing::get;

/// Run our app using a hyper server on http://localhost:3000.
#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app()).await.unwrap();
}

/// Create our application with one route that prints "Hello, World!"
pub fn app() -> axum::Router {
    axum::Router::new()
        .route("/", get(|| async { "Hello, World!" }))
}
```
