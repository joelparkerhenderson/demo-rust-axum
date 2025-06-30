# Uptime handler function

An axum route can call an axum handler, which is an async function that returns
anything that axum can convert into a response.

Edit file `main.rs`.

Add this anywhere before the function `main`:

```rust
/// Create the constant INSTANT so the program can track its own uptime.
pub static INSTANT: std::sync::LazyLock<std::time::Instant> = std::sync::LazyLock::new(|| std::time::Instant::now());
```

Edit file `main.rs` router code to add this route:

```rust
let app = axum::Router::new()
    …
    .route("/uptime",
        get(uptime)
    )
    …
```

Add a handler that returns the uptime as seconds, such as sixty seconds meaning one minute:

```rust
/// axum handler for "GET /uptime" which shows the program's uptime duration.
/// This shows how to write a handler that uses a global static lazy value.
pub async fn uptime() -> String {
    format!("{}", INSTANT.elapsed().as_secs())
}
```

## Try the demo

Shell:

```sh
cargo run
```

Browse <http://localhost:3000/uptime> or run:

```sh
curl 'http://localhost:3000/uptime'
```

You should see a web page that displays the uptime in seconds, such as sixty seconds meaning one minute:

```txt
60
```
