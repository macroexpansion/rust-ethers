use crate::airdrop_dictionary::AirdropDictionary;
use crate::signing::MessageSigner;
use anyhow::Result;
use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use rand::prelude::*;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tracing;
use tracing_subscriber;

lazy_static! {
    static ref DICTIONARY: AirdropDictionary = AirdropDictionary::load();
    static ref SIGNER: MessageSigner = MessageSigner::from_env("PRIVATE_KEY_LOCAL");
}

pub async fn app() {
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

#[derive(Deserialize, Debug)]
struct Payload {
    address: String,
}

#[derive(Serialize)]
struct Data {
    signature: String,
    item_ids: Vec<i32>,
    uris: Vec<String>,
}

#[derive(Serialize)]
struct Response {
    success: bool,
    message: String,
    data: Option<Data>,
}

#[tracing::instrument]
async fn sign(Json(payload): Json<Payload>) -> impl IntoResponse {
    let size = DICTIONARY.get(&payload.address);

    match size {
        Some(value) => {
            tracing::info!("signing");

            let (signature, item_ids, uris) = sign_message(payload.address, value).await.unwrap();

            let message = Response {
                success: true,
                message: "ok".to_string(),
                data: Some(Data {
                    signature,
                    item_ids,
                    uris,
                }),
            };

            return (StatusCode::OK, Json(message));
        }
        None => {
            tracing::info!("error");
            let error = Response {
                success: false,
                message: "address is not in the wait list".to_string(),
                data: None,
            };

            return (StatusCode::BAD_REQUEST, Json(error));
        }
    }
}

async fn sign_message(address: String, size: i32) -> Result<(String, Vec<i32>, Vec<String>)> {
    let item_ids = rand_item(size).unwrap();
    let uris: Vec<String> = item_ids
        .iter()
        .map(|&x| format!("https://static.howlcity.io/bike/{}.json", x))
        .collect();

    let message = MessageSigner::encode_message(&address, item_ids.clone(), uris.clone());
    let signature = SIGNER.create_signature(&message).await.unwrap();
    SIGNER.verify_signature(message, signature).unwrap();

    Ok((signature.to_string(), item_ids, uris))
}

fn rand_item(size: i32) -> Result<Vec<i32>> {
    let mut rng = thread_rng();
    let mut result: Vec<i32> = vec![];

    for _ in 0..size {
        let roll = rng.gen_range(101..=125);
        result.push(roll);
    }

    Ok(result)
}
