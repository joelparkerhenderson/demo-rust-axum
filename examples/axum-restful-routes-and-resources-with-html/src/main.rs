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

/// Run our app using a hyper server on http://localhost:3000.
#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app()).await.unwrap();
}

/// Create our application.
pub fn app() -> axum::Router {
    axum::Router::new()
        .route("/books",
            get(get_books)
            .post(post_books)
        )
        .route("/books/{id}",
            get(get_books_id)
            .put(put_books_id)
            .patch(patch_books_id)
            .delete(delete_books_id)
        )
        .route("/books/{id}/edit",
            get(get_books_id_with_edit_form)
            .patch(patch_books_id_with_edit_form)
        )
}

/// See file book.rs, which defines the `Book` struct.
mod book;
use crate::book::Book;

/// See file book_change.rs, which defines the `BookChange` struct.
mod book_change;
use crate::book_change::BookChange;

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
        println!("data: {:?}" , DATA.lock());
    }).join().unwrap()
}

/// axum handler for "GET /books" which responds with a resource page.
/// This demo uses our DATA; a production app could use a database.
/// This demo must clone the DATA in order to sort items by title.
pub async fn get_books() -> axum::response::Html<String> {
    thread::spawn(move || {
        match DATA.lock() {
            Ok(data) => {
        let mut books = data.values().collect::<Vec<_>>().clone();
        books.sort_by(|a, b| a.title.cmp(&b.title));
        books.iter().map(|&book|
            format!("<p>{}</p>\n", &book)
        ).collect::<String>()
    }).join().unwrap().into()
}

/// axum handler for "POST /books" which creates a new book resource.
/// This demo shows how axum can extract HTML data into a Book struct.
/// 
pub async fn post_books(
    axum::extract::Form(book_change): axum::extract::Form<Book>
) -> axum::response::Html<String> {
    thread::spawn(move || {
        match DATA.lock() {
            Ok(mut data) => {
        let id = data.keys().max().unwrap() + 1;
        let book = Book { id, ..book_change };
        data.insert(id, book.clone());
        format!("Post a new book with new id {}: {}", &id, &book)
    }).join().unwrap().into()
}

/// axum handler for "GET /books/{id}" which responds with one resource HTML page.
/// This demo app uses our crate::DATA variable, and iterates on it to find the id.
pub async fn get_books_id(
    axum::extract::Path(id): axum::extract::Path<u32>
) -> axum::response::Html<String> {
    thread::spawn(move || {
        match DATA.lock() {
            Ok(data) => {
        match data.get(&id) {
            Some(book) => format!("<p>{}</p>\n", &book),
            None => format!("<p>Book id {} not found</p>", id),
        }
    }).join().unwrap().into()
}

/// axum handler for "PUT /books/{id}" which sets a specific book resource.
/// This demo shows how axum can extract JSON data into a Book struct.
pub async fn put_books_id(
    axum::extract::Form(book): axum::extract::Form<Book>
) -> axum::response::Html<String> {
    thread::spawn(move || {
        match DATA.lock() {
            Ok(mut data) => {
        data.insert(book.id, book.clone());
        format!("Put book: {}", &book)
    }).join().unwrap().into()
}

/// axum handler for "DELETE /books/{id}" which destroys a resource.
/// This demo extracts an id, then deletes the book in the DATA store.
pub async fn delete_books_id(
    axum::extract::Path(id): axum::extract::Path<u32>
) -> axum::response::Html<String> {
    thread::spawn(move || {
        match DATA.lock() {
            Ok(mut data) => {
        if data.contains_key(&id) {
            data.remove(&id);
            format!("Delete book id: {}", &id)
        } else {
            format!("Book id not found: {}", &id)
        }
    }).join().unwrap().into()
}

/// axum handler for "PATCH /books/{id}" which updates attributes.
/// This demo shows how to mutate the book attributes in the DATA store.
pub async fn patch_books_id(
    axum::extract::Form(book_change): axum::extract::Form<BookChange>
) -> axum::response::Html<String> {
    thread::spawn(move || {
        let id = book_change.id;
        match DATA.lock() {
            Ok(mut data) => {
        if let Some(resource) = data.get_mut(&id) {
            if let Some(title) = book_change.title {
                resource.title = title;
            }
            if let Some(author) = book_change.author {
                resource.author = author;
            }
            format!("Patch book id: {}", &id)
        } else {
            format!("Book id not found: {}", &id)
        }
    }).join().unwrap().into()
}

/// axum handler for "GET /books/{id}/edit" which responds with a form.
/// This demo shows how to write a typical HTML form with input fields.
pub async fn get_books_id_with_edit_form(
    axum::extract::Path(id): axum::extract::Path<u32>
) -> axum::response::Html<String> {
    thread::spawn(move || {
        match DATA.lock() {
            Ok(data) => {
        match data.get(&id) {
            Some(book) => format!(
                concat!(
                    "<form method=\"patch\" action=\"/books/{}/edit\">\n",
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

/// axum handler for "PATCH /books/{id}/edit" which updates attributes.
/// This demo shows how to do HTML form submission then update attributes.
pub async fn patch_books_id_with_edit_form(
    form: axum::extract::Form<BookChange>
) -> axum::response::Html<String> {
    let book_change: BookChange = form.0;
    thread::spawn(move || {
        let id = book_change.id;
        match DATA.lock() {
            Ok(mut data) => {
        if data.contains_key(&id) {
            if let Some(title) = book_change.title {
                data.get_mut(&id).unwrap().title = title.clone();
            }
            if let Some(author) = book_change.author {
                data.get_mut(&id).unwrap().title = author.clone();
            }
            format!("Patch book id: {}", &id)
        } else {
            format!("Book id not found: {}", &book_change.id)
        }
    }).join().unwrap().into()
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum_test::TestServer;

    #[tokio::test]
    async fn get_books() {
        let server = TestServer::new(app()).unwrap();
        server.get("/books").await.assert_text("<p>Antigone by Sophocles</p>\n<p>Beloved by Toni Morrison</p>\n<p>Candide by Voltaire</p>\n");
    }

    // #[tokio::test]
    // async fn post_books() {
    //     let server = TestServer::new(app()).unwrap();
    //     let data = [
    //         ["title", "Decameron"],
    //         ["author", "Giovanni Boccaccio"],
    //     ];
    //     server.post("/books").form(&data).await.assert_text("Post a new book with new id 4: title: Decameron, author: Giovanni Boccaccio")
    // }

    // #[tokio::test]
    // async fn get_books_id() {
    //     let server = TestServer::new(app()).unwrap();
    //     //TODO
    //     server.get("/books/1").await.assert_text("<p>Antigone by Sophocles</p>");
    // }

    // #[tokio::test]
    // async fn put_books_id() {
    //     let server = TestServer::new(app()).unwrap();
    //     //TODO
    //     //server.put("/books/1").await.assert_text("<p>Antigone by Sophocles</p>");
    // }

    // #[tokio::test]
    // async fn patch_books_id() {
    //     let server = TestServer::new(app()).unwrap();
    //     //TODO
    //     //server.patch("/books/1").await.assert_text("<p>Antigone by Sophocles</p><p>Beloved by Toni Morrison</p><p>Candide by Voltaire</p>");
    // }

    // #[tokio::test]
    // async fn delete_books_id() {
    //     let server = TestServer::new(app()).unwrap();
    //     server.delete("/books/1").await.assert_text("Delete book id: \"1\"");
    // }

    // #[tokio::test]
    // async fn get_books_id_with_edit_form() {
    //     let server = TestServer::new(app()).unwrap();
    //     //TODO
    //     //server.get("/books/1/edit").await.assert_text("<p>Antigone by Sophocles</p><p>Beloved by Toni Morrison</p><p>Candide by Voltaire</p>");
    // }

    // #[tokio::test]
    // async fn patch_books_id_with_edit_form() {
    //     let server = TestServer::new(app()).unwrap();
    //     //TODO
    //     //server.get("/books/1/edit").await.assert_text("<p>Antigone by Sophocles</p><p>Beloved by Toni Morrison</p><p>Candide by Voltaire</p>");
    // }

}
