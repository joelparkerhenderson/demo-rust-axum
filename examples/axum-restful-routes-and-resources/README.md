# RESTful routes and resources

This section demonstrates how to:

- Create a book struct

- Create the data store

- Use the data store

- Get all books

- Post a new book

- Get one book

- Put one book

- Patch one book

- Delete one book

- Get one book as a web form

- Patch one book as a web form

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
// A production app could prefer an id to be type u32, UUID, etc.
#[derive(Debug, Deserialize, Clone, Eq, Hash, PartialEq)]
pub struct Book {
    pub id: String,
    pub title: String,
    pub author: String,
}
```

Add code to implement `Display`:

```rust
// Display the book using the format "{title} by {author}".
// This is a typical Rust trait and is not axum-specific.
impl std::fmt::Display for Book {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{} by {}",
            &self.title,
            &self.author,
        )
    }
}
```

Edit file `main.rs`.

Add code to include the `book` module and use the `Book` struct:

```rust
/// See file book.rs, which defines the `Book` struct.
mod book;
use crate::book::Book;
```

---

## Create the data store

For a production app, we could implement the data by using a database.

For this demo, we will implement the data by using a global variable `DATA`.

Edit file `Cargo.toml`.

Add the dependency `once_cell` which is for our global variables:

```toml
# Single assignment cells and lazy values.
once_cell = "1.10.0"
```

Create file `data.rs`.

Add this code:

```rust
// Use Lazy for creating a global variable e.g. our DATA.
use std::sync::LazyLock;

/// Use Mutex for thread-safe access to a variable e.g. our DATA data.
use std::sync::Mutex;

/// Create a data store as a global variable with `Lazy` and `Mutex`.
/// This demo implementation uses a `HashMap` for ease and speed.
/// The map key is a primary key for lookup; the map value is a Book.
static DATA: LazyLock<Mutex<HashMap<u32, Book>>> = LazyLock::new(|| 
    Mutex::new(
        HashMap::from([
            (1, Book {
                id: 1,
                title: "Antigone".into(),
                author: "Sophocles".into()
            }),
            (2, Book {
                id: 2, title:
                "Beloved".into(),
                author: "Toni Morrison".into()
            }),
            (3, Book {
                id: 3, title:
                "Candide".into(),
                author: "Voltaire".into()
            }),
        ])
    )
);
```

## Use the data store

Edit file `main.rs`.

Add code to include the `data` module and use the `DATA` global variable:

```rust
/// See file data.rs, which defines the DATA global variable.
mod data;
use crate::data::DATA;

/// Use Thread for spawning a thread e.g. to acquire our DATA mutex lock.
use std::thread;

/// To access data, create a thread, spawn it, then get the lock.
/// When you're done, then join the thread with its parent thread.
async fn print_data() {
    thread::spawn(move || {
        let data = DATA.lock().unwrap();
        println!("data: {:?}" ,data);
    }).join().unwrap()
}
```

If you want to see all the data now, then add function to `main`:

```rust
async fn main() {
    print_data().await;
    …
```

## Try the demo

Shell:

```sh
cargo run
```

Output:

```stdout
data: {
    1: Book { id: 1, title: "Antigone", author: "Sophocles" },
    2: Book { id: 2, title: "Beloved", author: "Toni Morrison" },
    3: Book { id: 3, title: "Candide", author: "Voltaire" }
}
```

---

## Get all books

Edit file `main.rs`.

Add a route:

```rust
let app = Router::new()
    …
    .route("/books",
        get(get_books)
    );
```

Add a handler:

```rust
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
```

## Try the demo

Shell:

```sh
cargo run
```

Shell:

```sh
curl 'http://localhost:3000/books'
```

Output:

```stdout
<p>Antigone by Sophocles</p>
<p>Beloved by Toni Morrison</p>
<p>Candide by Voltaire</p>
```

---

## Post a new book

Edit file `main.rs`.

Modify the route `/books` to append the function `post`:

```rust
let app = Router::new()
    …
    .route("/books",
        get(get_books)
        .post(post_books)
    );
```

Add a handler:

```rust
/// axum handler for "POST /books" which creates a new book resource.
/// This demo shows how axum can extract JSON data into a Book struct.
pub async fn post_books(
    axum::extract::Json(book): axum::extract::Json<Book>
) -> axum::response::Html<String> {
    thread::spawn(move || {
        let mut data = DATA.lock().unwrap();
        let id = data.keys().max().unwrap() + 1;
        let book = Book { id, ..book };
        data.insert(id, book.clone());
        format!("Post a new book with new id {}: {}", &id, &book)
    }).join().unwrap().into()
}
```

## Try the demo

Shell:

```sh
cargo run
```

Shell:

```sh
curl \
--request POST 'http://localhost:3000/books/0' \
--header "Content-Type: application/json" \
--data '{"id":0,"title":"Decameron","author":"Giovanni Boccaccio"}'
```

Output:

```stdout
Post a new book with new id 4: Decameron by Giovanni Boccaccio
```

Shell:

```sh
curl 'http://localhost:3000/books'
```

Output:

```stdout
<p>Antigone by Sophocles</p>
<p>Beloved by Toni Morrison</p>
<p>Candide by Voltaire</p>
<p>Decameron by Giovanni Boccaccio</p>
```

---

## Get one book

Edit file `main.rs`.

Add a route:

```rust
let app = Router::new()
    …
    .route("/books/{id}",
        get(get_books_id)
    );
```

Add a handler:

```rust
/// axum handler for "GET /books/{id}" which responds with one resource HTML page.
/// This demo app uses our DATA variable, and iterates on it to find the id.
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
```

## Try the demo

Shell:

```sh
cargo run
```

Shell:

```sh
curl 'http://localhost:3000/books/1'
```

Output:

```stdout
<p>Antigone by Sophocles</p>
```

Shell:

```sh
curl 'http://localhost:3000/books/0'
```

Output:

```stdout
<p>Book id 0 not found</p>
```

---

## Put one book

Edit file `main.rs`.

Modify the route `/books/{id}` to append the function `put`:

```rust
let app = Router::new()
    …
    .route("/books/{id}",
        get(get_books_id)
        .put(put_books_id)
    );
```

Add a handler:

```rust
/// axum handler for "PUT /books/{id}" which sets a specific book resource.
/// This demo shows how axum can extract JSON data into a Book struct.
pub async fn put_books_id(
    axum::extract::Json(book): axum::extract::Json<Book>
) -> axum::response::Html<String> {
    thread::spawn(move || {
        let mut data = DATA.lock().unwrap();
        data.insert(book.id, book.clone());
        format!("Put book: {}", &book)
    }).join().unwrap().into()
}
```

## Try the demo

Shell:

```sh
cargo run
```

Shell:

```sh
curl 'http://localhost:3000/books/5'
```

Output:

```stdout
<p>Book id 5 not found</p>
```

Shell:

```sh
curl \
--request PUT 'http://localhost:3000/books/4' \
--header "Content-Type: application/json" \
--data '{"id":5,"title":"Emma","author":"Jane Austen"}'
```

Output:

```stdout
Put book: Emma by Jane Austen
```

Shell:

```sh
curl 'http://localhost:3000/books/5'
```

Output:

```stdout
<p>Antigone by Sophocles</p>
<p>Beloved by Toni Morrison</p>
<p>Candide by Voltaire</p>
<p>Decameron by Giovanni Boccaccio</p>
<p>Emma by Jane Austen</p>
```

---

## Delete one book

Edit file `main.rs`.

Modify the route `/books/{id}` to append the function `delete`:

```rust
let app = Router::new()
    …
    .route("/books/{id}",
        get(get_books_id)
        .delete(delete_books_id)
    );
```

Add a handler:

```rust
/// axum handler for "DELETE /books/{id}" which destroys a resource.
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
```

## Try the demo

Shell:

```sh
cargo run
```

Shell:

```sh
curl --request DELETE 'http://localhost:3000/books/1'
```

Output:

```sh
Delete book id: 1
```

Shell:

```sh
curl 'http://localhost:3000/books'
```

Output:

```stdout
<p>Beloved by Toni Morrison</p>
<p>Candide by Voltaire</p>
```

---

## Patch one book

Create file `book_patch.rs`.

Add code for a BookPatch struct that has optional attributes:

```rust
// Use Deserialize to convert e.g. from request JSON into Book struct.
use serde::Deserialize;

// Demo book patch structure with some example fields for id, title, author.
// A production app could prefer an id to be type u32, UUID, etc.
#[derive(Debug, Deserialize, Clone, Eq, Hash, PartialEq)]
pub struct BookPatch {
    pub id: u32,
    pub title: Option<String>,
    pub author: Option<String>,
}
```

Add code to implement `Display`:

```rust
// Display the book using the format "{title} by {author}".
// This is a typical Rust trait and is not axum-specific.
impl std::fmt::Display for BookPatch {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{:?} by {:?}",
            self.title,
            self.author,
        )
    }
}
```

Edit file `main.rs`.

Add code to use the new BookPatch:

```rust
/// See file book_patch.rs, which defines the `BookPatch` struct.
mod book_patch;
use crate::book_patch::BookPatch;
```

Modify the route `/books/{id}` to append the function `patch`:

```rust
let app = Router::new()
    …
    .route("/books/{id}",
        get(get_books_id)
        .put(put_books_id)
        .patch(patch_books_id)
    );
```

Add a handler:

```rust
/// axum handler for "PATCH /books/{id}" which updates attributes.
/// This demo shows how to mutate the book attributes in the DATA store.
pub async fn patch_books_id(
    axum::extract::Json(book_patch): axum::extract::Json<BookPatch>
) -> axum::response::Html<String> {
    thread::spawn(move || {
        let id = book_patch.id;
        let mut data = DATA.lock().unwrap();
        if data.contains_key(&id) {
            if let Some(title) = book_patch.title {
                data.get_mut(&id).unwrap().title = title.clone();
            }
            if let Some(author) = book_patch.author {
                data.get_mut(&id).unwrap().title = author.clone();
            }
            format!("Patch book id: {}", &id)
        } else {
            format!("Book id not found: {}", &id)
        }
    }).join().unwrap().into()
}
```

## Try the demo

Shell:

```sh
cargo run
```

Shell:

```sh
curl 'http://localhost:3000/books/1'
```

Output:

```stdout
<p>Antigone by Sophocles</p>
```

Shell:

```sh
curl \
--request PATCH 'http://localhost:3000/books/1' \
--header "Content-Type: application/json" \
--data '{"id":1,"title":"Elektra"}'
```

Shell:

```sh
curl 'http://localhost:3000/books/1'
```

Output:

```stdout
<p>Elektra by Sophocles</p>
```

---

## Get one book as a web form

Edit file `main.rs`.

Add a route:

```rust
let app = Router::new()
    …
    .route("/books/{id}/form",
        get(get_books_id_form)
    );
```

Add a handler:

```rust
/// axum handler for "GET /books/{id}/form" which responds with a form.
/// This demo shows how to write a typical HTML form with input fields.
pub async fn get_books_id_form(
    axum::extract::Path(id): axum::extract::Path<u32>
) -> axum::response::Html<String> {
    thread::spawn(move || {
        let data = DATA.lock().unwrap();
        match data.get(&id) {
            Some(book) => format!(
                concat!(
                    "<form method=\"patch\" action=\"/books/{}/form\">\n",
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
```

## Try the demo

Shell:

```sh
cargo run
```

Shell:

```sh
curl 'http://localhost:3000/books/1/form'
```

Output:

```stdout
<form method="post" action="/books/1/form">
<p><input name="title" value="Antigone"></p>
<p><input name="author" value="Sophocles"></p>
<input type="submit" value="Save">
</form>
```

---

## Patch one book as a web form

Edit file `main.rs`.

Modify the route `/books/{id}/form` to append the function `post`:

```rust
let app = Router::new()
    …
    .route("/books/{id}/form",
        get(get_books_id_form)
        .patch(patch_books_id_form)
    );
```

Add a handler:

```rust
/// axum handler for "PATCH /books/{id}/form" which submits an HTML form.
/// This demo shows how to do a form submission then patch a resource.
pub async fn patch_books_id_form(
    form: axum::extract::Form<Book>
) -> axum::response::Html<String> {
    let new_book: Book = form.0;
    thread::spawn(move || {
        let mut data = DATA.lock().unwrap();
        if data.contains_key(&new_book.id) {
            if !new_book.title.is_empty() {
                data.get_mut(&new_book.id).unwrap().title = new_book.title.clone();
            }
            if !new_book.author.is_empty() {
                data.get_mut(&new_book.id).unwrap().author = new_book.author.clone();
            }
            format!("Patch book: {}", &new_book)
        } else {
            format!("Book id not found: {}", &new_book.id)
        }
    }).join().unwrap().into()
}
```

## Try the demo

Shell:

```sh
cargo run
```

Shell:

```sh
curl 'http://localhost:3000/books/1'
```

Output:

```stdout
<p>Antigone by Sophocles</p>
```

Shell:

```sh
curl \
--request PATCH 'http://localhost:3000/books/1/edit' \
--header "Content-Type: application/x-www-form-urlencoded" \
--data 'id=1&title=Elektra'
```

Shell:

```sh
curl 'http://localhost:3000/books/1'
```

Output:

```stdout
<p>Elektra by Sophocles</p>
```
