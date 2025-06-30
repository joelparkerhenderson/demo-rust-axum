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

- [Hello, World](examples/axum-hello-world/README.md)

- [Fallback router](examples/axum-fallback-router/README.md)

- [Graceful shutdown](examples/axum-graceful-shutdown/README.md)

**Examples:**

- [Hello handler function](examples/axum-hello-handler-function/README.md)

- [Epoch handler function](examples/axum-epoch-handler-function/README.md)

- [Uptime handler function](examples/axum-uptime-handler-function/README.md)

- [Count handler function](examples/axum-count-handler-function/README.md)

## Create axum routes and axum handlers

This section shows how to:

- [Respond with a string of HTML](examples/axum-respond-with-a-string-of-html/README.md)

- [Respond with a file of HTML](examples/axum-respond-with-a-file-of-html/README.md)

- [Respond with HTTP status code](examples/axum-respond-with-http-status-code-ok/README.md)

- [Respond with the request URI](examples/axum-respond-with-the-request-uri/README.md)

- [Respond with a custom header and a custom image](examples/axum-respond-with-a-custom-header-and-a-custom-image/README.md)

- [Respond with a JSON payload](examples/axum-respond-with-a-json-payload/README.md)

- [Respond to multiple HTTP verbs](examples/axum-respond-to-multiple-http-verbs/README.md)

## Extractors

An axum "extractor" is how you pick apart the incoming request in order to get
any parts that your handler needs.

This section shows how to:

- [Extract path parameters](examples/axum-extract-path-parameters/README.md)

- [Extract query parameters](examples/axum-extract-query-parameters/README.md)

- [Extract JSON parameters](examples/axum-extract-json-parameters-parameters/README.md)

**More:**

- [RESTful routes and resources](examples/axum-restful-routes-and-resources/README.md)

- [Tracing subscriber](examples/axum-tracing-subscriber/README.md)

- [Bind + host + port + socket address](examples/axum-bind-host-port-socket-address/README.md)

**Ending:**

- [axum repository examples](doc/97-axum-repository-examples.md)

- [Ideas for next steps](doc/98-ideas-for-next-steps.md)

- [Conclusion](doc/99-conclusion.md)


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
