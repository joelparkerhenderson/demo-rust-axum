/// Run our app using a hyper server on http://localhost:3000.
#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app()).await.unwrap();
}
/// Create our application.
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
    async fn test() {
        let app: axum::Router = app();
        let server = TestServer::new(app).unwrap();
        let response_text = server.get("/").await.text();
        assert_eq!(response_text, "Hello, World!");
    }
}
