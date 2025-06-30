//! Demo of Rust and axum web framework.
//!
//! <https://github.com/joelparkerhenderson/demo-rust-axum>
//!
//! For more see the file `README.md` in the project root.

/// Use axum capabilities.
use axum::routing::get;

#[tokio::main]
async fn main() {
    // Build our application by creating our router.
    let app = axum::Router::new()
        .route("/demo-form",
        get(get_demo_form)
            .post(post_demo_form)
        );

    // Run our application as a hyper server on http://localhost:3000.
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

/////
/// Demo HTML form GET and POST
/////

/// See file book.rs, which defines the `Book` struct.
mod book;
use crate::book::Book;

/// axum handler for "GET /demo-form" which responds with a form.
/// This demo shows how to write a typical HTML form with input fields.
pub async fn get_demo_form() ->  axum::response::Html<&'static str> {
    r#"
    <!doctype html>
    <html>
        <head>
            <title>Book Form</title>
        </head>
        <body>
            <h1>Book Form</h1>
            <form method="post" action="/demo-form">
                <p>
                    <label for="title">
                        Title:
                        <br>
                        <input name="title">
                    </label>
                </p>
                <p>
                    <label for="author">
                        Author:
                        <br>
                        <input name="author">
                    </label>
                </p>
                <p>
                    <input type="submit">
                </p?
            </form>
        </body>
    </html>
    "#.into()
}

/// axum handler for "POST /demo-form" which submits an HTML form.
/// This demo shows how extract a form submission to a struct.
pub async fn post_demo_form(
    form: axum::extract::Form<Book>
) -> axum::response::Html<String> {
    let book: Book = form.0;
    format!(
        r#"
        <!doctype html>
        <html>
            <head>
                <title>Book</title>
            </head>
            <body>
                <h1>Book</h1>
                {:?}
            </body>
        </html>
        "#,
        &book
    ).into()
}
