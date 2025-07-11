# Hello, World! with an axum test server

The crate `axum-test` enables easy testing of an axum app by running the axum app within an axum test server.

Edit file `Cargo.toml`.

Use this kind of package and these dependencies, including the dev-dependency using the crate `axum-text`:

```toml
[dev-dependencies]
axum-test = { version = "17.3.0" } # Library for writing tests for web servers written using Axum.
```

Edit file `src/main.rs`.

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use axum_test::TestServer;

    #[tokio::test]
    async fn test() {
        let server = TestServer::new(app()).unwrap();
        server.get("/").await.assert_text("Hello, World!");
    }
}
```

## Try the demo

Shell:

```sh
cargo test
```

Output:

```stdout
running 1 test
test tests::test ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```
