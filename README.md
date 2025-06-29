# Demo of Rust and axum web framework

Demonstration of:

- [Rust](https://www.rust-lang.org): programming language that focuses on reliability and stability.

- [axum](https://crates.io/crates/axum): web framework that focuses on ergonomics and modularity.

- [tower](https://crates.io/crates/tower): library for building robust clients and servers.

- [hyper](https://hyper.rs/): fast and safe HTTP library for the Rust language.

- [tokio](https://tokio.rs): platform for writing asynchronous I/O backed applications.

- [Serde](https://crates.io/crates/serde): serialization/deserialization framework.

## Feedback

Have an idea, suggestion, or feedback? Let us know via GitHub issues.

Have a code improvement or bug fix? We welcome GitHub pull requests.

## License

This demo uses the license Creative Commons Share-and-Share-Alike.

## Thanks

Thanks to all the above projects and their authors. Donate to them if you can.

Does this demo help your work? Donate here if you can via GitHub sponsors.

## Contact

Have feedback? Have thoughts about this? Want to contribute?

Contact the maintainer at <joel@joelparkerhenderson.com>.

## Table of Contents

**Start Here:**

- [What is this?](doc/01-what-is-this.md)
- [What is axum?](doc/02-what-is-axum.md)
- [What is Tower?](doc/03-what-is-tower.md)
- [What is Hyper?](doc/04-what-is-hyper.md)
- [What is Tokio?](doc/05-what-is-tokio.md)
- [What is Serde?](doc/06-what-is-serde.md)

**Examples:**

- [Hello, World](examples/hello-world/README.md)
- [Fallback router](examples/fallback-router/README.md)
- [Graceful shutdown](examples/graceful-shutdown/README.md)

**Handlers:**

- [Hello handler function](examples/hello-handler-function/README.md)
- [Epoch handler function](examples/epoch-handler-function/README.md)
- [Uptime handler function](examples/uptime-handler-function/README.md)
- [Count handler function](examples/count-handler-function/README.md)

**Respond:**

- [Respond with HTTP status code](examples/respond-with-http-status-code/README.md)
- [Respond with string of HTML](examples/respond-with-string-of-html/README.md)
- [Respond with file of HTML](examples/respond-with-file-of-html/README.md)
- [Respond with custom header and custom image](examples/respond-with-custom-header-and-custom-image/README.md)

**More:**

- [RESTful routes and resources](examples/restful-routes-and-resources/README.md)
- [Tracing subscriber](examples/tracing-subscriber/README.md)
- [Bind + host + port + socket address](examples/bind-host-port-socket-address/README.md)

**Ending:**

- [axum repository examples](doc/97-axum-repository-examples.md)
- [Ideas for next steps](doc/98-ideas-for-next-steps.md)
- [Conclusion](doc/99-conclusion.md)

## Create axum routes and axum handlers

This section shows how to:

- Respond with HTML text

- Respond with an HTML file

- Respond with HTTP status code OK

- Respond with the request URI

- Respond with a custom header and image

- Respond to multiple HTTP verbs

## Try the demo

Shell:

```sh
cargo run
```

Browse <http://localhost:3000/demo.html>

You should see HTML with headline text "Hello".


## Respond with the request URI

Edit file `main.rs`.

Add a route:

```rust
let app = axum::Router::new()
    …
    .route("/demo-uri",
        get(demo_uri)
    );
```

Add a handler:

```rust
/// axum handler for "GET /demo-uri" which shows the request's own URI.
/// This shows how to write a handler that receives the URI.
pub async fn demo_uri(uri: axum::http::Uri) -> String {
    format!("The URI is: {:?}", uri)
}
```

## Try the demo

Shell:

```sh
cargo run
```

Browse <http://localhost:3000/demo-uri>

You should see "The URI is: /demo-uri!".



- [Respond with file of HTML](examples/respond-with-file-of-html/README.md)


## Respond to multiple HTTP verbs

axum routes can use HTTP verbs, including GET, PUT, PATCH, POST, DELETE.

Edit file `main.rs`.

Add axum routes for each HTTP verb:

```rust
let app = axum::Router::new()
    …
    .route("/foo",
        get(get_foo)
        .put(put_foo)
        .patch(patch_foo)
        .post(post_foo)
        .delete(delete_foo),
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

## Try the demo

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

The command `curl` uses GET by default, i.e. these are equivalent:

```sh
curl 'http://localhost:3000/foo'

curl --request GET 'http://localhost:3000/foo'
```

## Extractors

An axum "extractor" is how you pick apart the incoming request in order to get
any parts that your handler needs.

This section shows how to:

- Extract path parameters

- Extract query parameters

- Extract a JSON payload

- Respond with a JSON payload

### Caution: extractors syntax change

Before you read the next section, please be aware the extractor syntax has changed between axum 0.7 and 0.8, approximately as of 2025-01-01.

- Axum 0.7 extractor syntax uses a colon like this: `:foo`.

- Axum 0.8 extractor syntax uses curly braces such as `{foo}`.

If you try to use the older syntax, you will get an error message such as: "Path segments must not start with `:`. For capture groups, use `{capture}`. If you meant to literally match a segment starting with a colon, call `without_v07_checks` on the router."

### Extractor path parameters

Add a route using path parameter syntax, such as "{id}", in order to tell axum to
extract a path parameter and deserialize it into a variable named `id`.

Edit file `main.rs`.

Add a route:

```rust
let app = axum::Router::new()
    …
    .route("/items/{id}",
        get(get_items_id)
    );
```

Add a handler:

```rust
/// axum handler for "GET /items/{id}" which uses `axum::extract::Path`.
/// This extracts a path parameter then deserializes it as needed.
pub async fn get_items_id(
    axum::extract::Path(id):
        axum::extract::Path<String>
) -> String {
    format!("Get items with path id: {:?}", id)
}
```

## Try the demo

Shell:

```sh
cargo run
```

Shell:

```sh
curl 'http://localhost:3000/items/1'
```

Ouput:

```sh
Get items with id: 1
```

## Extract query parameters

Edit file `main.rs`.

Add code to use HashMap to deserialize query parameters into a key-value map:

```rust
use std::collections::HashMap;
```

Add a route:

```rust
let app = axum::Router::new()
    …
    .route("/items",
        get(get_items)
    );
```

Add a handler:

```rust
/// axum handler for "GET /items" which uses `axum::extract::Query`.
/// This extracts query parameters and creates a key-value pair map.
pub async fn get_items(
    axum::extract::Query(params):
        axum::extract::Query<HashMap<String, String>>
) -> String {
    format!("Get items with query params: {:?}", params)
}
```

## Try the demo

Shell:

```sh
cargo run
```

Shell:

```sh
curl 'http://localhost:3000/items?a=b'
```

Output:

```sh
Get items with query params: {"a": "b"}
```

## Respond with a JSON payload

The axum extractor for JSON can help with a response, by formatting JSON data
then setting the response application content type.

Edit file `main.rs`.

Add code to use Serde JSON:

```rust
/// Use Serde JSON to serialize/deserialize JSON, such as in a request.
/// axum creates JSON or extracts it by using `axum::extract::Json`.
/// For this demo, see functions `get_demo_json` and `post_demo_json`.
use serde_json::{json, Value};
```

Add a route:

```rust
let app = axum::Router::new()
    …
    .route("/demo.json",
        get(get_demo_json)
    );
```

Add a handler:

```rust
/// axum handler for "GET /demo.json" which uses `axum::extract::Json`.
/// This buffers the request body then deserializes it bu using serde.
/// The `Json` type supports types that implement `serde::Deserialize`.
pub async fn get_demo_json() -> axum::extract::Json<Value> {
    json!({"a":"b"}).into()
}
```

## Try the demo

Shell:

```sh
cargo run
```

To request JSON with curl, set a custom HTTP header like this:

```sh
curl \
--header "Accept: application/json" \
--request GET 'http://localhost:3000/demo.json'
```

Output:

```sh
{"a":"b"}
```

## Extract a JSON payload

The axum extractor for JSON deserializes a request body into any type that
implements `serde::Deserialize`. If the extractor is unable to parse the request
body, or if the request is missing the header `Content-Type: application/json`,
 then the extractor returns HTTP `BAD_REQUEST` (404).

Edit file `main.rs`.

Modify the route `/demo.json` to append the function `put`:

```rust
let app = axum::Router::new()
    …
    .route("/demo.json",
        get(get_demo_json)
        .put(put_demo_json)
    )
```

Add a handler:

```rust
/// axum handler for "PUT /demo.json" which uses `axum::extract::Json`.
/// This buffers the request body then deserializes it using serde.
/// The `Json` type supports types that implement `serde::Deserialize`.
pub async fn put_demo_json(
    axum::extract::Json(data): axum::extract::Json<serde_json::Value>
) -> String{
    format!("Put demo JSON data: {:?}", data)
}
```

## Try the demo

Shell:

```sh
cargo run
```

Send the JSON:

```sh
curl \
--request PUT 'http://localhost:3000/demo.json' \
--header "Content-Type: application/json" \
--data '{"a":"b"}'
```

Output:

```sh
Put demo JSON data: Object({"a": String("b")})
```


## Render HTML

Edit the file `main.rs`. 

At the bottom, add this:

```rust
////
/// HTML rendering helpers.
////

/// Render strings into an HTML table tag.
pub fn html_table_tag(table: Vec<Vec<String>>) -> String {
    format!("<table>\n{}</table>\n", html_table_tr_tags(table))
}

/// Render strings into HTML table tr tags.
pub fn html_table_tr_tags(rows: Vec<Vec<String>>) -> String {
    rows.iter()
        .map(|row| 
            format!("<tr>{}</tr>\n", html_table_td_tags(row))
        )
        .collect::<String>()
}

/// Render strings into HTML table td tags.
pub fn html_table_td_tags(cells: &Vec<String>) -> String {
    cells.iter().map(|cell| 
        format!("<td>{}</td>", cell)
    ).collect::<String>()
}
```

Edit the function `get_books` so it uses the new HTML helpers:

```rust
pub async fn get_books() -> axum::response::Html<String> {
    thread::spawn(move || {
        let data = DATA.lock().unwrap();
        let mut books = data.values().collect::<Vec<_>>().clone();
        books.sort_by(|a, b| 
            a.title.as_deref().unwrap_or("").cmp(b.title.as_deref().unwrap_or(""))
        );
        let table: Vec<Vec<String>> = books.iter().map(|person| 
            vec![
                book.id.clone(),
                book.title,
                book.author,
            ]
        ).collect();
        html_table(table)
    })
    .join()
    .unwrap()
    .into()
}
```
