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
        .route("/request-uri",
            get(request_uri)
        )
}

/// axum handler for "GET /request-uri" which shows the request's own URI.
/// This shows how to write a handler that receives the URI.
pub async fn request_uri(uri: axum::http::Uri) -> String {
    format!("The URI is: {:?}", uri)
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum_test::TestServer;

    #[tokio::test]
    async fn test() {
        let server = TestServer::new(app()).unwrap();
        server.get("/request-uri").await.assert_text("The URI is: http://localhost/request-uri");
    }
}
