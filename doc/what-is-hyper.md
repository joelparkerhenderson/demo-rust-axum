## What is hyper?

[hyper](https://hyper.rs/) is a fast HTTP implementation written in and for Rust.

* A Client for talking to web services.

* A Server for building those web services.

* Blazing fast* thanks to Rust.

* High concurrency with non-blocking sockets.

* HTTP/1 and HTTP/2 support.


### Hyper is low-level

hyper is a relatively low-level library, meant to be a building block for libraries and applications.

If you are looking for a convenient HTTP client, then you may wish to consider [reqwest](https://github.com/seanmonstar/reqwest).

If you are looking for a convenient HTTP server, then you may wish to consider [warp](https://github.com/seanmonstar/warp).

Both are built on top of hyper.


### Hello, World!

```rust
use std::convert::Infallible;

async fn handle(
    _: hyper::Request<Body>
) -> Result<hyper::Response<hyper::Body>, Infallible> {
    Ok(hyper::Response::new("Hello, World!".into()))
}

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    let make_svc = hyper::service::make_service_fn(|_conn| async {
        Ok::<_, Infallible>(hyper::service::service_fn(handle))
    });

    let server = hyper::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(make_svc);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
```


<div style="page-break-before:always;"></div>


