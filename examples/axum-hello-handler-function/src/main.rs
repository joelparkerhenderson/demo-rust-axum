use axum::routing::get;

/// Run our application as a hyper server on http://localhost:3000.
#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app()).await.unwrap();
}

/// Create our application.
pub fn app() -> axum::Router {
    axum::Router::new()
    .route("/",
        get(hello)
    )
}

/// axum handler for "GET /" which returns a string and causes axum to
/// immediately respond with status code `200 OK` and with the string.
pub async fn hello() -> String {
   "Hello, World!".into()
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum_test::TestServer;

    #[tokio::test]
    async fn response() {
        let server = TestServer::new(app()).unwrap();
        server.get("/").await.assert_text("Hello, World!");
    }
}
