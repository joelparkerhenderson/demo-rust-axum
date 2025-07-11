/// Use axum routing.
use axum::routing::get;

/// Run our app using a hyper server on http://localhost:3000.
#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app()).await.unwrap();
}

/// Create our application with one route that prints "Hello, World!"
pub fn app() -> axum::Router {
    axum::Router::new()
    .route("/",
        get(hello)
    )
}

/// axum handler function which returns a string and causes axum to
/// immediately respond with status code `200 OK` and the string.
pub async fn hello() -> String {
   "Hello, World!".into()
}
