//! Demo of Rust and axum web framework.
//!
//! <https://github.com/joelparkerhenderson/demo-rust-axum>
//!
//! For more see the file `README.md` in the project root.
//!
//!
//! The axum route has functions for HTTP verbs:
//! GET, PUT, PATCH, POST, DELETE.
//!
//! We use the naming convention of `get_foo`, `put_foo`, etc.
//!
//! These demo handlers in this section simply return a string.

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
        .route("/foo",
            get(get_foo)
            .put(put_foo)
            .patch(patch_foo)
            .post(post_foo)
            .delete(delete_foo)
        )
}

/// axum handler for "GET /foo" which returns a string message.
/// This shows our naming convention for HTTP GET handlers.
pub async fn get_foo() -> String {
    "GET foo".to_string()
}

/// axum handler for "PUT /foo" which returns a string message.
/// This shows our naming convention for HTTP PUT handlers.
pub async fn put_foo() -> String {
    "PUT foo".to_string()
}

/// axum handler for "PATCH /foo" which returns a string message.
/// This shows our naming convention for HTTP PATCH handlers.
pub async fn patch_foo() -> String {
    "PATCH foo".to_string()
}

/// axum handler for "POST /foo" which returns a string message.
/// This shows our naming convention for HTTP POST handlers.
pub async fn post_foo() -> String {
    "POST foo".to_string()
}

/// axum handler for "DELETE /foo" which returns a string message.
/// This shows our naming convention for HTTP DELETE handlers.
pub async fn delete_foo() -> String {
    "DELETE foo".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum_test::TestServer;

    #[tokio::test]
    async fn get_foo() {
        let server = TestServer::new(app()).unwrap();
        server.get("/foo").await.assert_text("GET foo");
    }

    #[tokio::test]
    async fn put_foo() {
        let server = TestServer::new(app()).unwrap();
        server.put("/foo").await.assert_text("PUT foo");
    }

    #[tokio::test]
    async fn patch_foo() {
        let server = TestServer::new(app()).unwrap();
        server.patch("/foo").await.assert_text("PATCH foo");
    }

    #[tokio::test]
    async fn post_foo() {
        let server = TestServer::new(app()).unwrap();
        server.post("/foo").await.assert_text("POST foo");
    }

    #[tokio::test]
    async fn delete_foo() {
        let server = TestServer::new(app()).unwrap();
        server.delete("/foo").await.assert_text("DELETE foo");
    }
    
}
