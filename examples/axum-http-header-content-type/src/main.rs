//! Demo of Rust and axum web framework.
//!
//! <https://github.com/joelparkerhenderson/demo-rust-axum>
//!
//! For more see the file `README.md` in the project root.

/// Use axum capabilities.
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
        .route("/demo-css",
        get(get_demo_css)
        )
        .route("/demo-csv",
            get(get_demo_csv)
        )
}

/// axum handler for "GET /demo-css" to get cascading style sheet text.
/// This sets a custom header "text/css" then sends the CSS text.
/// Browsers many handle CSS text in different ways, such as displaying
/// the text using colors, or downloading the CSS as a file, etc.
async fn get_demo_css() -> impl axum::response::IntoResponse {
    let mut headers = axum::http::HeaderMap::new();
    headers.insert(
        axum::http::header::CONTENT_TYPE,
        axum::http::HeaderValue::from_static(&"text/css")
    );
    (
        headers,
        concat!(
            "b: { font-color: red; }\n",
            "i: { font-color: blue; }\n",
        )
    )
}

/// axum handler for "GET /demo-csv" to get comma separated values text.
/// This sets a custom header "text/csv" then sends the CSV text.
/// Browsers many handle CSV text in different ways, such as displaying
/// the text using a data table, or downloading the CSV as a file, etc.
async fn get_demo_csv() -> impl axum::response::IntoResponse {
    let mut headers = axum::http::HeaderMap::new();
    headers.insert(
        axum::http::header::CONTENT_TYPE,
        axum::http::HeaderValue::from_static(&"text/csv")
    );
    (
        headers,
        concat!(
            "alfa,bravo,charlie\n",
            "delta,echo,foxtrot\n",
        )
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum_test::TestServer;

    #[tokio::test]
    async fn get_demo_css() {
        let server = TestServer::new(app()).unwrap();
        let response: axum_test::TestResponse = server.get("/demo-css").await;
        response.assert_header("content-type", "text/css");
        response.assert_text("b: { font-color: red; }\ni: { font-color: blue; }\n");
    }

    #[tokio::test]
    async fn get_demo_csv() {
        let server = TestServer::new(app()).unwrap();
        let response = server.get("/demo-csv").await;
        response.assert_header("content-type", "text/csv");
        response.assert_text("alfa,bravo,charlie\ndelta,echo,foxtrot\n");
    }

}
