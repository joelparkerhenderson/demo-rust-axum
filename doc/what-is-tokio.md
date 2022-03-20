## What is tokio?

[tokio](https://tokio.rs/) is an asynchronous runtime for the Rust programming language.

* Building blocks for writing network applications.

* Flexibility to target a wide range of systems.

* Memory-safe, thread-safe, and misuse-resistant.

The tokio stack includes:

* Runtime: Including I/O, timer, filesystem, synchronization, and scheduling.

* Hyper: An HTTP client and server library supporting HTTP protocols 1 and 2.

* Tonic: A boilerplate-free gRPC client and server library for network APIS.

* Tower: Modular components for building reliable clients and servers.

* Mio: Minimal portable API on top of the operating-system's evented I/O API.

* Tracing: Unified, structured, event-based, data collection and logging.

* Bytes: A rich set of utilities for manipulating byte arrays.


### Demo tokio server

```rust
#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    loop {
        let (socket, _address) = listener.accept().await.unwrap();
        tokio::spawn(async move {
            process(socket).await;
        });
    }
}

async fn process(socket: tokio::net::TcpStream) {
    println!("process socket");
}
```

### Demo tokio client

```rust
#[tokio::main]
async fn main() -> Result<()> {
    let mut client = client::connect("127.0.0.1:3000").await?;
    println!("connected);
    Ok(())
}
```
