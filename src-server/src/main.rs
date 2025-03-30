use axum::{
    routing::get,
    Router,
    response::IntoResponse,
    extract::Query,
};
use platform_core::domain::usecase::greet;
use serde::Deserialize;
use tower_http::cors;
use tracing;
use tracing_subscriber;

#[derive(Debug, Deserialize)]
struct GreetParams {
    name: String,
}

async fn greet_handler(Query(params): Query<GreetParams>) -> impl IntoResponse {
    tracing::info!("Greet handler called with params: {:?}", params);
    let message = greet(&params.name);
    message
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    tracing::info!("Starting server");
    let cors = cors::CorsLayer::new()
        .allow_origin(cors::Any)
        .allow_methods(cors::Any)
        .allow_headers(cors::Any);

    let app = Router::new().route("/greet", get(greet_handler)).layer(cors);

    println!("Server running");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
