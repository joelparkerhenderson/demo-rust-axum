//! Demo of Rust and axum web framework.
//!
//! <https://github.com/joelparkerhenderson/demo-rust-axum>
//!
//! For more see the file `README.md` in the project root.

/// Use axum capabities.
use axum::routing::get;
use axum::handler::Handler;

/// Use HashMap to deserialize a HTTP GET query into a key-value map.
/// axum extracts query parameters by using `axum::extract::Query`.
/// For the implementation, see function `get_query`.
use std::collections::HashMap;

/// Use Serde JSON to serialize/deserialize JSON, such as in a request.
/// axum creates JSON or extracts it by using `axum::extract::Json`.
/// For this demo, see functions `get_demo_json` and `post_demo_json`.
use serde_json::{json, Value};

#[tokio::main]
async fn main() {

    // Build our application by creating our router.
    let app = axum::Router::new()
        .route("/books",
        get(get_books)
            .put(put_books)
        );

    // Run our application as a hyper server on http://localhost:3000.
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

/// Tokio signal handler that will wait for a user to press CTRL+C.
/// We use this in our hyper `Server` method `with_graceful_shutdown`.
async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("Expect shutdown signal handler");
    println!("signal shutdown");
}

/////
/// Demo HTML form GET and POST
/////

/// See file book.rs, which defines the `Book` struct.
mod book;
use crate::book::Book;

/// axum handler for "GET /form" which responds with a form.
/// This demo shows how to write a typical HTML form with input fields.
pub async fn get_form(
    axum::extract::Path(id): axum::extract::Path<u32>
) ->  axum::response::Html<&'static str> {
    r#"
    <!doctype html>
    <html>
        <head>
            <title>Book Form</title>
        </head>
        <body>
            <h1>Book Form</h1>
            <form method=\"post\" action="/form">
                <label for="title">
                    Title:
                    <br>
                    <input name="title">
                </label>
                <label for="author">
                    Author:
                    <br>
                    <input name="author">
                </label>
                <input type="submit">
            </form>
        </body>
    </html>
    "#.into()
}

/// axum handler for "POST /form" which submits an HTML form.
/// This demo shows how extract a form submission to a struct.
pub async fn post_books_id_form(
    form: axum::extract::Form<Book>
) -> axum::response::Html<String> {
    format!("Post book: {}", &new_book)
    .into()
}
