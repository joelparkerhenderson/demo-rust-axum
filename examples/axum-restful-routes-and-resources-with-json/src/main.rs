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
use axum::{
    routing::get, 
    response::IntoResponse,
};

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

/// axum handler for "GET /books" which responds with a resource page.
/// This demo uses our DATA; a production app could use a database.
/// This demo must clone the DATA in order to sort items by title.
pub async fn get_books() -> axum::response::Response {
    thread::spawn(move || {
        match DATA.lock() {
            Ok(data) => {
                let mut books = data.values().collect::<Vec<_>>().to_owned();
                books.sort_by(|a, b| a.title.cmp(&b.title));
                (axum::http::StatusCode::OK, axum::response::Json(books)).into_response()
            },
            Err(_) => {
                axum::http::StatusCode::INTERNAL_SERVER_ERROR.into_response()
            }
        }
    }).join().unwrap().into()
}

/// axum handler for "POST /books" which creates a new book resource.
/// This demo shows how axum can extract JSON data into a Book struct.
pub async fn post_books(
    axum::extract::Json(book): axum::extract::Json<Book>
) -> axum::http::StatusCode {
    thread::spawn(move || {
        match DATA.lock() {
            Ok(mut data) => {
                let id = data.keys().max().unwrap() + 1;
                let book = Book { id, ..book };
                data.insert(id, book.clone());
                axum::http::StatusCode::CREATED
            },
            Err(_) => {
                axum::http::StatusCode::INTERNAL_SERVER_ERROR
            }
        }
    }).join().unwrap().into()
}

/// axum handler for "GET /books/{id}" which responds with one resource HTML page.
/// This demo app uses our crate::DATA variable, and iterates on it to find the id.
pub async fn get_books_id(
    axum::extract::Path(id): axum::extract::Path<u32>
) -> axum::response::Response {
    thread::spawn(move || {
        match DATA.lock() {
            Ok(data) => {
                match data.get(&id) {
                    Some(book) => (axum::http::StatusCode::OK, axum::response::Json(book)).into_response(),
                    None => http::StatusCode::NOT_FOUND.into_response()
                }
            },
            Err(_) => {
                axum::http::StatusCode::INTERNAL_SERVER_ERROR.into_response()
            }
        }
    }).join().unwrap().into()
}

/// axum handler for "PUT /books/{id}" which sets a specific book resource.
/// This demo shows how axum can extract JSON data into a Book struct.
pub async fn put_books_id(
    axum::extract::Json(book): axum::extract::Json<Book>
) -> axum::http::StatusCode {
    thread::spawn(move || {
        match DATA.lock() {
            Ok(mut data) => {
                data.insert(book.id, book.clone());
                axum::http::StatusCode::CREATED
            },
            Err(_) => {
                axum::http::StatusCode::INTERNAL_SERVER_ERROR
            }
        }
    }).join().unwrap().into()
}

/// axum handler for "DELETE /books/{id}" which destroys a resource.
/// This demo extracts an id, then deletes the book in the DATA store.
pub async fn delete_books_id(
    axum::extract::Path(id): axum::extract::Path<u32>
) -> axum::http::StatusCode {
    thread::spawn(move || {
        match DATA.lock() {
            Ok(mut data) => {
                if data.contains_key(&id) {
                    data.remove(&id);
                    axum::http::StatusCode::NO_CONTENT
                } else {
                    axum::http::StatusCode::NOT_FOUND
                }
            },
            Err(_) => {
                axum::http::StatusCode::INTERNAL_SERVER_ERROR
            }
        }
    }).join().unwrap().into()
}

/// axum handler for "PATCH /books/{id}" which updates attributes.
/// This demo shows how to mutate the book attributes in the DATA store.
pub async fn patch_books_id(
    axum::extract::Json(book_change): axum::extract::Json<BookChange>
) -> axum::http::StatusCode {
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
                    axum::http::StatusCode::NO_CONTENT
                } else {
                    axum::http::StatusCode::NOT_FOUND
                }
            },
            Err(_) => {
                axum::http::StatusCode::INTERNAL_SERVER_ERROR
            }
        }
    }).join().unwrap().into()
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum_test::TestServer;
    use serde_json::json;

    #[tokio::test]
    async fn get_books() {
        let server = TestServer::new(app()).unwrap();
        server.get("/books").await.assert_json(&json!(
            [
                {
                    "id": 1,
                    "title": "Antigone", 
                    "author": "Sophocles"
                },
                {
                    "id": 2,
                    "title": "Beloved", 
                    "author": "Toni Morrison"
                },
                {
                    "id": 3,
                    "title": "Candide", 
                    "author": "Voltaire"
                }
            ]
        ));
    }

    #[tokio::test]
    async fn post_books() {
        let server = TestServer::new(app()).unwrap();
        let j = json!(
            {
                "title": "Decameron",
                "author": "Giovanni Boccaccio"
            }
        );
        server.post("/books").json(&j).await.assert_status(axum::http::StatusCode::CREATED);
    }

    #[tokio::test]
    async fn get_books_id() {
        let server = TestServer::new(app()).unwrap();
        server.get("/books/1").await.assert_json(&json!(
            {
                "id": 1,
                "title": "Antigone", 
                "author": "Sophocles"
            }
        ));
    }

    #[tokio::test]
    async fn put_books_id() {
        let server = TestServer::new(app()).unwrap();
        let j = json!(
            {
                "title": "Decameron",
                "author": "Giovanni Boccaccio"
            }
        );
        server.put("/books/4").json(&j).await.assert_status(axum::http::StatusCode::CREATED);
    }

    #[tokio::test]
    async fn patch_books_id() {
        let server = TestServer::new(app()).unwrap();
        let j = json!(
            {
                "id": 1,
                "title": "Elektra"
            }
        );
        server.patch("/books/1").json(&j).await.assert_status(axum::http::StatusCode::NO_CONTENT);
    }

    #[tokio::test]
    async fn delete_books_id() {
        let server = TestServer::new(app()).unwrap();
        server.delete("/books/1").await.assert_status(axum::http::StatusCode::NO_CONTENT);
    }

}
