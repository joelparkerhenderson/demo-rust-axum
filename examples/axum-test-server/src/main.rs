#[tokio::main]
async fn main() {
    // Run our application as a hyper server on http://localhost:3000.
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app()).await.unwrap();
}

pub fn app() -> axum::Router {
    axum::Router::new()
    .route("/",
        axum::routing::get(|| async { "Hello, World!" })
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum_test::TestServer;

    #[tokio::test]
    async fn response_text() {
        let app: axum::Router = app();
        let server = TestServer::new(app).unwrap();
        let response_text = server.get("/").await.text();
        assert_eq!(response_text, "Hello, World!");
    }
}
