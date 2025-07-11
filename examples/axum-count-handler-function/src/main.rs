use axum::routing::get;

/// Create the atomic variable COUNT so the program can track its own count.
pub static COUNT: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);

/// Run our app using a hyper server on http://localhost:3000.
#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app()).await.unwrap();
}

/// Create our application.
pub fn app() -> axum::Router {
    axum::Router::new()
        .route("/count",
            get(count)
        )
}

/// axum handler for "GET /count" which shows the program's count duration.
/// This shows how to write a handler that uses a global static lazy value.
pub async fn count() -> String {
    COUNT.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    format!("{}", COUNT.load(std::sync::atomic::Ordering::SeqCst))
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum_test::TestServer;

    #[tokio::test]
    async fn test() {
        let server = TestServer::new(app()).unwrap();
        let response_text_0 = server.get("/count").await.text();
        let response_text_1 = server.get("/count").await.text();
        assert!(response_text_0 < response_text_1, "{} < {}", response_text_0, response_text_1);
    }
}
