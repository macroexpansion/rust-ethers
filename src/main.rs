use airdrop_cmc::server::sign;
use axum::{
    routing::{get, post},
    Router,
};
use std::net::SocketAddr;
use tracing;
use tracing_subscriber;

#[tokio::main]
async fn main() {
    app().await;
}

async fn app() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", get(root))
        .route("/sign", post(sign));

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
