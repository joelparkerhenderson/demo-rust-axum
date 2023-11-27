
## Graceful shutdown

We want our demo server to be able to do graceful shutdown.

  * [Read tokio documentation about graceful shutdown](https://tokio.rs/tokio/topics/shutdown)

  * [Read hyper documentation about graceful shutdown](https://hyper.rs/guides/server/graceful-shutdown/)

Tokio graceful shutdown generally does these steps:

  * Find out when to shut down.

  * Tell each part of the program to shut down.

  * Wait for each part of the program to shut down.

Hyper graceful shutdown generally does these steps:

  * The server stops accepting new requests.

  * The server waits for all in-progress requests to complete.

  * Then the server shuts down.

Edit file `main.rs`.

Create a tokio signal handler that listens for a user pressing CTRL+C:

```rust
/// Tokio signal handler that will wait for a user to press CTRL+C.
/// We use this in our hyper `Server` method `with_graceful_shutdown`.
async fn shutdown_signal() {
   tokio::signal::ctrl_c().await {
        Ok(()) => {},
        Err(err) => {
            eprintln!("Unable to listen for shutdown signal: {}", err);
        },
    }    
}
```

Modify the `axum::serve` code to add the method `with_graceful_shutdown`:

```rust
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await.unwrap();
```


### Try the demo…

Shell:

```sh
cargo run
```

Browse <http://localhost:3000>

You should see "Hello, World!".

In your shell, press CTRL-C.

Your shell should print "^Csignal shutdown" or possibly just "Csignal shutdown".


<div style="page-break-before:always;"></div>

