//! Demo of Rust and axum web framework.
//!
//! <https://github.com/joelparkerhenderson/demo-rust-axum>
//!
//! For more see the file `README.md` in the project root.

/// Use axum capabities.
use axum::routing::get;

#[tokio::main]
async fn main() {
    // Build our application by creating our router.
    let app = axum::Router::new()
        .route("/demo-css",
        get(get_demo_css)
        )
        .route("/demo-csv",
            get(get_demo_csv)
        );

    // Run our application as a hyper server on http://localhost:3000.
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

/// axum handler for "GET /demo-css" to get cascading style sheet text.
/// This sets a custom header "text/css" then sends the CSS text.
/// Browsers many handle CSS text in different ways, such as displaying
/// the text using colors, or downloading the CSS as a file, etc.
async fn get_demo_css() -> impl axum::response::IntoResponse { 
    (
        axum::response::Headers([
            (
                axum::http::header::CONTENT_TYPE, 
                "text/css"
            ),
        ]),
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
    (
        axum::response::Headers([
            (
                axum::http::header::CONTENT_TYPE, 
                "text/csv"
            ),
        ]),
        concat!(
            "alpha,bravo,charlie\n",
            "delta,echo,foxtrot\n",
        )
    )
}
