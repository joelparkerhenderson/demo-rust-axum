# Tracing subscriber

Edit file `Cargo.toml`.

Add dependencies:

```toml
tracing = { version = "~0.1.41" } #  Application-level tracing for Rust.
tracing-subscriber = { version = "~0.3.19", features = ["env-filter"] } # Utilities for `tracing` subscribers.
```

Edit file `main.rs`.

Add code to use tracing:

```rust
/// Use tracing crates for application-level tracing output.
use tracing_subscriber::{
    layer::SubscriberExt,
    util::SubscriberInitExt,
};
```

Add a tracing subscriber to the start of the main function:

```rust
/// Run our main function.
#[tokio::main]
pub async fn main() {
    // Start tracing and emit a tracing event.
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();
    tracing::event!(tracing::Level::INFO, "main");
```

## Try the demo

Shell:

```sh
cargo run
```

You should see console output that shows tracing initialization such as:

```sh
2024-08-31T20:06:34.894986Z  INFO demo_rust_axum: tracing
```
