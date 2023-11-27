## What is axum?

[axum](https://crates.io/crates/axum) is a web framework with high level features:

* Route requests to handlers with a macro free API.

* Declaratively parse requests using extractors.

* Simple and predictable error handling model.

* Generate responses with minimal boilerplate.

* Take full advantage of the tower and its ecosystem.


### How is axum special?

The tower ecosystem is what sets axum apart from other frameworks:

* axum doesn’t have its own middleware system but instead uses tower::Service.

* axum gets timeouts, tracing, compression, authorization, and more, for free.

* axum can share middleware with applications written using hyper or tonic.


### Why learn axum now?

* axum combines the speed and security of Rust with the power of
battle-tested libraries for middleware, asynchronous programming, and HTTP.

* axum is primed to reach developers who are currently using
other Rust web frameworks, such as Actix, Rocket, Warp, and others.

* axum is likely to appeal to programmers are seeking a faster
web framework and who want closer-to-the-metal capabilities. 


### Hello, World!

```rust
#[tokio::main]
async fn main() {
    // Build our application with a single route.
    let app = axum::Router::new().route("/",
        axum::handler::get(|| async { "Hello, World!" }));

    // Run our application as a hyper server on http://localhost:3000.
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await.unwrap();
}
```
