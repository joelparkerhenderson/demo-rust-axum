use axum::routing::get;

/// Create the constant INSTANT so the program can track its own uptime.
pub static INSTANT: std::sync::LazyLock<std::time::Instant> = std::sync::LazyLock::new(|| std::time::Instant::now());

#[tokio::main]
async fn main() {
    // Build our application with a single route.
    let app = axum::Router::new()
        .route("/uptime",
            get(uptime)
        );

    // Run our application as a hyper server on http://localhost:3000.
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

/// axum handler for "GET /uptime" which shows the program's uptime duration.
/// This shows how to write a handler that uses a global static lazy value.
pub async fn uptime() -> String {
    format!("{}", INSTANT.elapsed().as_secs())
}
