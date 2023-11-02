use askama::Template;
use axum::extract::Path;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::Router;
use std::net::SocketAddr;
use tower_http::services::ServeDir;
use fake::faker::name::raw::*;
use fake::locales::*;
use fake::Fake;


#[derive(Template)]
#[template(path = "index.html")]
struct HelloTemplate {
    name: String,
}
#[derive(Template)]
#[template(path = "wow.html")]
struct WowTemplate {
    message: String,
}

async fn hello(Path(name): Path<String>) -> impl IntoResponse {
    HelloTemplate { name }
}

async fn wow() -> impl IntoResponse {
    let message = Name(EN).fake();
    WowTemplate { message}
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/:name", get(hello))
        .route("/wow", get(wow))
        .nest_service("/static", ServeDir::new("static/"));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("🚀 Server started, listening on {addr}");
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("Failed to start server");
}
