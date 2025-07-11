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
        .route("/string.html",
            get(string_html)
        )
}

/// axum handler for "GET /string.html" which responds with a string.
/// The `Html` type sets an HTTP header content-type of `text/html`.
pub async fn string_html() -> axum::response::Html<&'static str> {
    "<html><body><h1>Headline</h1><p>Paragraph</b></body></html>".into()
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum_test::TestServer;

    #[tokio::test]
    async fn test() {
        let server = TestServer::new(app()).unwrap();
        server.get("/string.html").await.assert_text("<html><body><h1>Headline</h1><p>Paragraph</b></body></html>")
    }
}
