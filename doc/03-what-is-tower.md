# What is tower?

[Tower](https://tower.rs/) is a library of modular and reusable components for
building robust networking clients and servers.

Tower aims to make it as easy as possible to build robust networking clients and
servers. It is protocol agnostic, but is designed around a request / response
pattern. If your protocol is entirely stream based, Tower may not be a good fit.

## Service

At Tower's core is the Service trait. A Service is an asynchronous function that
takes a request and produces a response.

```rust
pub trait Service<Request> {
    type Response;
    type Error;
    type Future: Future<Output = Result<Self::Response, Self::Error>>;

    fn poll_ready(
        &mut self,
        cx: &mut Context<'_>,
    ) -> Poll<Result<(), Self::Error>>;

    fn call(&mut self, req: Request) -> Self::Future;
}
```

## Call

The most common way to call a service is:

```rust
use tower::{
    Service,
    ServiceExt,
};

let response = service
    // wait for the service to have capacity
    .ready().await?
    // send the request
    .call(request).await?;
```
