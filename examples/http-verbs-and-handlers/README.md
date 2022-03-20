## HTTP verbs and handlers

axum routes can use HTTP verbs, including GET, PUT, PATCH, POST, DELETE.

Edit file `main.rs`.

Add axum routes for each HTTP verb:

```rust
let app = Router::new()
    …
    .route("/foo",
        get(get_foo)
        .put(put_foo)
        .patch(patch_foo)
        .post(post_foo)
        .delete(delete_foo)
    )
```

Add axum handlers:

```rust
/// axum handler for "GET /foo" which returns a string message.
/// This shows our naming convention for HTTP GET handlers.
pub async fn get_foo() -> String {
   "GET foo".to_string()
}

/// axum handler for "PUT /foo" which returns a string message.
/// This shows our naming convention for HTTP PUT handlers.
pub async fn put_foo() -> String {
   "PUT foo".to_string()
}

/// axum handler for "PATCH /foo" which returns a string message.
/// This shows our naming convention for HTTP PATCH handlers.
pub async fn patch_foo() -> String {
   "PATCH foo".to_string()
}

/// axum handler for "POST /foo" which returns a string message.
/// This shows our naming convention for HTTP POST handlers.
pub async fn post_foo() -> String {
   "POST foo".to_string()
}

/// axum handler for "DELETE /foo" which returns a string message.
/// This shows our naming convention for HTTP DELETE handlers.
pub async fn delete_foo() -> String {
   "DELETE foo".to_string()
}
```


### Try the demo…

Shell:

```sh
cargo run
```

To make a request using an explicit request of GET or POST or DELETE,
one way is to use a command line program such as `curl` like this:

Shell:

```sh
curl --request GET 'http://localhost:3000/foo'
```

Output:

```sh
GET foo
```

Shell:

```sh
curl --request PUT 'http://localhost:3000/foo'
```

Output:

```sh
PUT foo
```

Shell:

```sh
curl --request PATCH 'http://localhost:3000/foo'
```

Output:

```sh
PATCH foo
```

Shell:

```sh
curl --request POST 'http://localhost:3000/foo'
```

Output:

```sh
POST foo
```

Shell:

```sh
curl --request DELETE 'http://localhost:3000/foo'
```

Output:

```sh
DELETE foo
```
