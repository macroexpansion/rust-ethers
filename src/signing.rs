use anyhow::Result;
use dotenv;
use ethers::{
    abi::{encode, token::Token},
    signers::{LocalWallet, Signer},
    types::{Signature, H160, U256},
};
use std::str::FromStr;

pub struct MessageSigner {
    wallet: LocalWallet,
}

impl MessageSigner {
    pub fn from(private_key: &str) -> Self {
        MessageSigner {
            wallet: LocalWallet::from_str(private_key).unwrap(),
        }
    }

    pub fn from_env(key: &str) -> Self {
        dotenv::dotenv().ok();
        dotenv::from_filename(".secret").ok();
        let private_key = dotenv::var(key).unwrap();

        MessageSigner {
            wallet: LocalWallet::from_str(&private_key).unwrap(),
        }
    }

    pub fn encode_message(address: &str, item_ids: Vec<i32>, uris: Vec<String>) -> Vec<u8> {
        let mut token_vec: Vec<Token> = Vec::new();

        let address: H160 = H160::from_str(address).unwrap();
        token_vec.push(Token::Address(address));

        for id in &item_ids {
            let id: String = id.to_string();
            let u256: U256 = U256::from_dec_str(&id).unwrap();
            token_vec.push(Token::Uint(u256));
        }

        for uri in &uris {
            token_vec.push(Token::String(uri.to_string()));
        }

        let encoded_message: Vec<u8> = encode(&token_vec);
        encoded_message
    }

    pub async fn create_signature(&self, message: &Vec<u8>) -> Result<Signature> {
        let signature: Signature = self.wallet.sign_message(&message).await?;
        // println!("Produced signature {}", signature.to_string());

        Ok(signature)
    }

    pub fn verify_signature(&self, message: Vec<u8>, signature: Signature) -> Result<()> {
        signature.verify(message, self.wallet.address()).unwrap();
        println!("Verified signature produced by {:?}", self.wallet.address());

        Ok(())
    }
}
