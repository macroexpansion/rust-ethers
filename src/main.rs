use airdrop_cmc::server::sign;
use axum::{
    http::Method,
    routing::{get, post},
    Router,
};
use clap::{ArgEnum, Parser};
use std::env;
use std::net::SocketAddr;
use tower::ServiceBuilder;
use tower_http::cors::{any, CorsLayer};
use tracing;
use tracing_subscriber;

#[derive(Parser, Debug)]
#[clap(about, version, author)]
struct Args {
    #[clap(arg_enum, short, long)]
    network: Network,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ArgEnum, Debug)]
enum Network {
    MAINNET,
    TESTNET,
    LOCAL,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let privatekey = match args.network {
        Network::MAINNET => "PRIVATE_KEY_MAINNET",
        Network::TESTNET => "PRIVATE_KEY_TESTNET",
        Network::LOCAL => "PRIVATE_KEY_LOCAL",
    };
    env::set_var("SIGNER_PRIVATE_KEY", privatekey);

    app().await;
}

async fn app() {
    tracing_subscriber::fmt::init();

    let cors = CorsLayer::new()
        .allow_methods(vec![Method::GET, Method::POST])
        .allow_headers(any())
        .allow_origin(any());

    let middleware_stack = ServiceBuilder::new().layer(cors);

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
