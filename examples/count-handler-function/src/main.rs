use axum::routing::get;

/// Create the atomic variable COUNT so the program can track its own count.
pub static COUNT: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);

#[tokio::main]
async fn main() {
    // Build our application with a single route.
    let app = axum::Router::new()
        .route("/count",
            get(count)
        );

    // Run our application as a hyper server on http://localhost:3000.
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

/// axum handler for "GET /count" which shows the program's count duration.
/// This shows how to write a handler that uses a global static lazy value.
pub async fn count() -> String {
    COUNT.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    format!("{}", COUNT.load(std::sync::atomic::Ordering::SeqCst))
}
