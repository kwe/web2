use askama::Template;
use axum::extract::Path;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::Router;
use std::net::SocketAddr;
use tower_http::services::ServeDir;

#[derive(Template)]
#[template(path = "index.html")]
struct HelloTemplate {
    name: String,
}

async fn hello(Path(name): Path<String>) -> impl IntoResponse {
    HelloTemplate { name }
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/:name", get(hello))
        .nest_service("/static", ServeDir::new("static/"));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("🚀 Server started, listening on {addr}");
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("Failed to start server");
}
