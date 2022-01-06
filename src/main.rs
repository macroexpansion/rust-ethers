// use axum::http::Method;
use airdrop_cmc::server::sign;
use axum::{
    routing::{get, post},
    Router,
    http::Method
};
use std::net::SocketAddr;
use tracing;
use tracing_subscriber;
use tower::{ServiceBuilder};
use tower_http::cors::{CorsLayer, any};

#[tokio::main]
async fn main() {
    app().await;
}

async fn app() {
    tracing_subscriber::fmt::init();

    let cors = CorsLayer::new()
        .allow_methods(vec![Method::GET, Method::POST])
        .allow_headers(any())
        .allow_origin(any());

    let middleware_stack = ServiceBuilder::new()
        .layer(cors);

    let app = Router::new()
        .route("/", get(root))
        .route("/sign", post(sign))
        .layer(middleware_stack);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> &'static str {
    "Hello, World!"
}
