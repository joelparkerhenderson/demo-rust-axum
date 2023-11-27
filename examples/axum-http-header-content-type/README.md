# axum HTTP header content type

To return a custom content type, you can set an axum HTTP header.


## Start

Start with the typical file `main.rs`.

```rust
#[tokio::main]
async fn main() {
    // Build our application by creating our router.
    let app = axum::Router::new();
    
    // Run our application as a hyper server on http://localhost:3000.
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
```


## Content type is cascading style sheet (CSS)

Add a route to demonstrate text that is formatted as a cascading style sheet (CSS):

```rust
    let app = axum::Router::new()
        .route("/demo-css",
            get(get_demo_css)
        );
```

Add handler:

```rust
/// axum handler for "GET /demo-css" to get cascading style sheet text.
/// This sets a custom HTTP header "text/css" then sends the CSS text.
/// Browsers many handle CSS text in different ways, such as displaying
/// the text using colors, or downloading the CSS as a file, etc.
async fn get_demo_css() -> impl axum::response::IntoResponse { 
    let mut headers = axum::http::HeaderMap::new();
    headers.insert(
        axum::http::header::CONTENT_TYPE, 
        axum::http::HeaderValue::from_static(&"text/css")
    );
    (
        headers,
        concat!(
            "b: { font-color: red; }\n",
            "i: { font-color: blue; }\n",
        )
    )
}
```


### Try the demo…

Shell:

```sh
cargo run
```

Browse <http://localhost:3000/demo-css>

You should see your browser receive the cascading style sheet (CSS),
and either display the text or prompt you to download a CSS file.

```text
b: { font-color: red; }
i: { font-color: blue; }
```


## Content type is comma separated values (CSV)

Append a route to demonstrate text that is formatted as comma separated values (CSV):

```rust
    let app = axum::Router::new()
        .route("/demo-css",
            get(get_demo_css)
        )
        .route("/demo-csv",
            get(get_demo_csv)
        );
```

Add handler:

```rust

/// axum handler for "GET /demo-csv" to get comma separated values text.
/// This sets a custom HTTP header "text/csv" then sends the CSV text.
/// Browsers many handle CSV text in different ways, such as displaying
/// the text using a data table, or downloading the CSV as a file, etc.
async fn get_demo_csv() -> impl axum::response::IntoResponse {
    let mut headers = axum::http::HeaderMap::new();
    headers.insert(
        axum::http::header::CONTENT_TYPE, 
        axum::http::HeaderValue::from_static(&"text/csv")
    );
    (
        headers,
        concat!(
            "alpha,bravo,charlie\n",
            "delta,echo,foxtrot\n",
        )
    )
}
```


### Try the demo…

Shell:

```sh
cargo run
```

Browse <http://localhost:3000/demo-csv>

You should see your browser receive the comma separated values (CSV),
and either display the text or prompt you to download a CSV file.

```text
alpha,bravo,charlie
delta,echo,foxtrot"
```
