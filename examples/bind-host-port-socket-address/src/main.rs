#[tokio::main]
async fn main() {
    // Build our application with a single route.
    let app = axum::Router::new()
        .route("/",
            axum::routing::get(|| async { "Hello, World!" })
        );

    // Bind using host & port & socket address
    let host = [127, 0, 0, 1];
    let port = 3000;
    let addr = std::net::SocketAddr::from((host, port));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
