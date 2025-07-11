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
        .route("/epoch",
            get(epoch)
        )
}

/// axum handler for "GET /epoch" which shows the current epoch time.
/// This shows how to write a handler that uses time and can error.
pub async fn epoch() -> Result<String, axum::http::StatusCode> {
    match std::time::SystemTime::now().duration_since(std::time::SystemTime::UNIX_EPOCH) {
        Ok(duration) => Ok(format!("{}", duration.as_secs())),
        Err(_) => Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum_test::TestServer;

    #[tokio::test]
    async fn test() {
        let server = TestServer::new(app()).unwrap();
        let response_text_0 = server.get("/epoch").await.text();
        std::thread::sleep(std::time::Duration::from_secs(1));
        let response_text_1 = server.get("/epoch").await.text();
        assert!(response_text_0 < response_text_1, "{} < {}", response_text_0, response_text_1)
    }
}
