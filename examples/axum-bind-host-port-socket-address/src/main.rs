/// Run our main function.
#[tokio::main]
async fn main() {
    // Bind using host & port & socket address
    let host = [127, 0, 0, 1];
    let port = 3000;
    let addr = std::net::SocketAddr::from((host, port));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
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
        let server = TestServer::new(app()).unwrap();
        server.get("/").await.assert_text("Hello, World!");
    }
}
