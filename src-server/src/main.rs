use axum::{
    routing::get,
    Router,
    response::IntoResponse,
    extract::Query,
};
use platform_core::domain::usecase::greet;
use serde::Deserialize;
use tower_http::cors;

#[derive(Deserialize)]
struct GreetParams {
    name: String,
}

async fn greet_handler(Query(params): Query<GreetParams>) -> impl IntoResponse {
    let message = greet(&params.name);
    message
}

#[tokio::main]
async fn main() {
    let cors = cors::CorsLayer::new()
        .allow_origin(cors::Any)
        .allow_methods(cors::Any)
        .allow_headers(cors::Any);

    let app = Router::new().route("/greet", get(greet_handler)).layer(cors);

    println!("Server running");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
