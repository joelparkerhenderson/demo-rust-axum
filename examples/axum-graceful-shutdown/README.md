# Graceful shutdown

We want our demo server to be able to do graceful shutdown.

- [Read tokio documentation about graceful shutdown](https://tokio.rs/tokio/topics/shutdown)

- [Read hyper documentation about graceful shutdown](https://hyper.rs/guides/server/graceful-shutdown/)

- [Read axum example about graceful shutdown](https://github.com/tokio-rs/axum/tree/main/examples/graceful-shutdown)

Tokio graceful shutdown generally does these steps:

- Find out when to shut down.

- Tell each part of the program to shut down.

- Wait for each part of the program to shut down.

Hyper graceful shutdown generally does these steps:

- The server stops accepting new requests.

- The server waits for all in-progress requests to complete.

- Then the server shuts down.

## Add a shutdown signal

Add a shutdown signal that handles a user pressing Ctrl+C and/or a Unix terminate signal.

Edit the file `main.rs`.

Add this section:

```rust
/// Shutdown signal to run axum with graceful shutdown when
/// a user presses Ctrl+C or Unix sends a terminate signal.
pub async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}
```

Edit the file `main.rs` line `axum::serve` to add the method `with_graceful_shutdown`:

```rust
axum::serve(listener, app)
    .with_graceful_shutdown(shutdown_signal())
    .await
    .unwrap();
```

## Try the demo

Shell:

```sh
cargo run
```

On your command line, press Ctrl+C.

You should see the server shut down.
