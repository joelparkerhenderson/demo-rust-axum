# HTML form GET and POST

This section demonstrates how to:

* Create a book struct

* Handle HTML GET for a new book form

* Handle HTML POST to extract a book struct


## Create a book struct

Suppose we want our app to have features related to books.

Create a new file `book.rs`.

Add code to use deserialization:

```rust
/// Use Deserialize to convert e.g. from request JSON into Book struct.
use serde::Deserialize;
```

Add code to create a book struct that derives the traits we want:

```rust
/// Demo book structure with some example fields for id, title, author.
#[derive(Debug, Deserialize, Clone, Eq, Hash, PartialEq)]
pub struct Book {
    pub title: String,
    pub author: String,
}
```


## Create the server


Edit file `main.rs`.

Create a typical server:

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

Add code to include the Book struct:

```rust
/////
/// Demo HTML form GET and POST
/////

/// See file book.rs, which defines the `Book` struct.
mod book;
use crate::book::Book;
```


## Create HTML form GET

Edit file `main.rs`.

Add the route `/demo-form` with the function `get`:

```rust
async fn main() {
    let app = axum::Router::new()
        .route("/demo-form",
            get(get_demo_form)
        );
```

Add handler:

```rust
/// axum handler for "GET /demo-form" which responds with a form.
/// This demo shows how to write a typical HTML form with input fields.
pub async fn get_demo_form() ->  axum::response::Html<&'static str> {
    r#"
    <!doctype html>
    <html>
        <head>
            <title>Book</title>
        </head>
        <body>
            <h1>Book</h1>
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
```


### Try the demo…

Shell:

```sh
cargo run
```

Browse <http://localhost:3000/demo-form>

You should see a HTML form with fields for title and author, and a button to submit the form.


## Create HTML form POST

Edit file `main.rs`.

Modify the route `/demo-form` to append the function `post`:

```rust
async fn main() {
    let app = axum::Router::new()
        .route("/demo-form",
            get(get_demo_form)
            .post(post_demo_form)
        );
```

Add handler:

```rust
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
```


### Try the demo…

Shell:

```sh
cargo run
```

Browse <http://localhost:3000/demo-form>

You should see the book form. 

Fill in some information, such as a title and author. 

Example:

  * Title: Antigone

  * Author: Sophocles

Tap the button, or click the button, to submit the form via HTTP POST.

You should see the HTML POST response, which shows the information you posted.

Example response output:

```txt
Book { title: "Antigone", author: "Sophocles" } 
```
