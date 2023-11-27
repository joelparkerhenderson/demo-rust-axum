//! Demo of Rust and axum web framework.
//!
//! <https://github.com/joelparkerhenderson/demo-rust-axum>
//!
//! For more see the file `README.md` in the project root.
//!
//! This example uses a `Book` struct, a `DATA` variable
//! that is a lazy mutex global variable, and handlers
//! that process the routes for HTTP verbs GET, PUT, etc.

/// Use axum capabilities.
use axum::routing::get;

#[tokio::main]
async fn main() {
    // Build our application by creating our router.
    let app = axum::Router::new()
        .route("/books",
        get(get_books)
            .put(put_books)
        )
        .route("/books/:id",
            get(get_books_id)
            .delete(delete_books_id)
        )
        .route("/books/:id/form",
            get(get_books_id_form)
            .post(post_books_id_form)
        );

    // Run our application as a hyper server on http://localhost:3000.
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

/// See file book.rs, which defines the `Book` struct.
mod book;
use crate::book::Book;

/// See file data.rs, which defines the DATA global variable.
mod data;
use crate::data::DATA;

/// Use Thread for spawning a thread e.g. to acquire our crate::DATA mutex lock.
use std::thread;

/// To access data, create a thread, spawn it, then get the lock.
/// When you're done, then join the thread with its parent thread.
#[allow(dead_code)]
async fn print_data() {
    thread::spawn(move || {
        let data = DATA.lock().unwrap();
        println!("data: {:?}" ,data);
    }).join().unwrap()
}

/// axum handler for "GET /books" which responds with a resource page.
/// This demo uses our DATA; a production app could use a database.
/// This demo must clone the DATA in order to sort items by title.
pub async fn get_books() -> axum::response::Html<String> {
    thread::spawn(move || {
        let data = DATA.lock().unwrap();
        let mut books = data.values().collect::<Vec<_>>().clone();
        books.sort_by(|a, b| a.title.cmp(&b.title));
        books.iter().map(|&book|
            format!("<p>{}</p>\n", &book)
        ).collect::<String>()
    }).join().unwrap().into()
}

/// axum handler for "PUT /books" which creates a new book resource.
/// This demo shows how axum can extract JSON data into a Book struct.
pub async fn put_books(
    axum::extract::Json(book): axum::extract::Json<Book>
) -> axum::response::Html<String> {
    thread::spawn(move || {
        let mut data = DATA.lock().unwrap();
        data.insert(book.id, book.clone());
        format!("Put book: {}", &book)
    }).join().unwrap().into()
}

/// axum handler for "GET /books/:id" which responds with one resource HTML page.
/// This demo app uses our crate::DATA variable, and iterates on it to find the id.
pub async fn get_books_id(
    axum::extract::Path(id): axum::extract::Path<u32>
) -> axum::response::Html<String> {
    thread::spawn(move || {
        let data = DATA.lock().unwrap();
        match data.get(&id) {
            Some(book) => format!("<p>{}</p>\n", &book),
            None => format!("<p>Book id {} not found</p>", id),
        }
    }).join().unwrap().into()
}

/// axum handler for "DELETE /books/:id" which destroys a resource.
/// This demo extracts an id, then mutates the book in the DATA store.
pub async fn delete_books_id(
    axum::extract::Path(id): axum::extract::Path<u32>
) -> axum::response::Html<String> {
    thread::spawn(move || {
        let mut data = DATA.lock().unwrap();
        if data.contains_key(&id) {
            data.remove(&id);
            format!("Delete book id: {}", &id)
        } else {
            format!("Book id not found: {}", &id)
        }
    }).join().unwrap().into()
}

/// axum handler for "GET /books/:id/form" which responds with a form.
/// This demo shows how to write a typical HTML form with input fields.
pub async fn get_books_id_form(
    axum::extract::Path(id): axum::extract::Path<u32>
) -> axum::response::Html<String> {
    thread::spawn(move || {
        let data = DATA.lock().unwrap();
        match data.get(&id) {
            Some(book) => format!(
                concat!(
                    "<form method=\"post\" action=\"/books/{}/form\">\n",
                    "<input type=\"hidden\" name=\"id\" value=\"{}\">\n",
                    "<p><input name=\"title\" value=\"{}\"></p>\n",
                    "<p><input name=\"author\" value=\"{}\"></p>\n",
                    "<input type=\"submit\" value=\"Save\">\n",
                    "</form>\n"
                ),
                &book.id,
                &book.id,
                &book.title,
                &book.author
            ),
            None => format!("<p>Book id {} not found</p>", id),
        }
    }).join().unwrap().into()
}

/// axum handler for "POST /books/:id/form" which submits an HTML form.
/// This demo shows how to do a form submission then update a resource.
pub async fn post_books_id_form(
    form: axum::extract::Form<Book>
) -> axum::response::Html<String> {
    let new_book: Book = form.0;
    thread::spawn(move || {
        let mut data = DATA.lock().unwrap();
        if data.contains_key(&new_book.id) {
            data.insert(new_book.id, new_book.clone());
            format!("Post book: {}", &new_book)
        } else {
            format!("Book id not found: {}", &new_book.id)
        }
    }).join().unwrap().into()
}
