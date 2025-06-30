# Bind & host & port & socket address

To bind the server listener, one way is to specify the host and port as a string such as:

```rust
let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
```

If you prefer to bind by using a host variable, a port variable, and a socket
address variable, then you can edit file `main.rs`.

Modify the listener code to use whatever host and port and socket address you want:

```rust
let host = [127, 0, 0, 1];
let port = 3000;
let addr = std::net::SocketAddr::from((host, port));
let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
```
