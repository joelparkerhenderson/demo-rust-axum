//! Demo of Rust and axum web framework.
//!
//! <https://github.com/joelparkerhenderson/demo-rust-axum>
//!
//! For more see the file `README.md` in the project root.

/// Use axum capabilities.
use axum::routing::get;

/// Run our app using a hyper server on http://localhost:3000.
#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app()).await.unwrap();
}

/// Create our application.
pub fn app() -> axum::Router {
    axum::Router::new()
    .route("/demo-form",
        get(get_demo_form)
            .post(post_demo_form)
        )
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

#[cfg(test)]
mod tests {
    use super::*;
    use axum_test::TestServer;

    #[tokio::test]
    async fn get_demo_form() {
        let server = TestServer::new(app()).unwrap();
        let response_text = server.get("/demo-form").await.text();
        assert!(response_text.contains("<form method=\"post\" action=\"/demo-form\">"));
    }

    #[tokio::test]
    async fn post_demo_form() {
        let server = TestServer::new(app()).unwrap();
        let data = [
            ["title", "alfa"],
            ["author", "bravo"],
        ];
        let response_text = server.post("/demo-form").form(&data).await.text();
        assert!(response_text.contains("Book { title: \"alfa\", author: \"bravo\" }"));
    }
}
