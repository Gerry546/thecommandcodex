
use axum::{
    routing::get,
    Router,
};

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new()
        .route("/", get(homepage))
        .route("/about", get(about_page));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// Homepage handler
async fn homepage() -> &'static str {
    "Welcome to My Rust Website!"
}
// About page handler
async fn about_page() -> &'static str {
    "This is the About Page of the Rust Website."
}
