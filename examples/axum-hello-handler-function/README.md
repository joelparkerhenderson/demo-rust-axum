# Handler function

An axum route can call an axum handler, which is an async function that returns
anything that axum can convert into a response.

Edit file `main.rs`.

Our demos will often use the axum routing `get` function, so add code to use it:

```rust
use axum::routing::get;
```

Edit file `main.rs` and the router code to this:

```rust
let app = axum::Router::new()
    .route("/",
        get(hello)
    );
```

Add a handler, which is an async function that returns a string:

```rust
/// axum handler for "GET /" which returns a string and causes axum to
/// immediately respond with status code `200 OK` and with the string.
pub async fn hello() -> String {
   "Hello, World!".into()
}
```

## Try the demo

Shell:

```sh
cargo run
```

Browse <http://localhost:3000> or run:

```sh
curl 'http://localhost:3000'
```

Output:

```sh
Hello, World!
```

In your shell, press CTRL-C to shut down.
