use anyhow::Result;
use dotenv;
use ethers::{
    abi::{encode, token::Token},
    signers::{LocalWallet, Signer},
    types::{Signature, H160, U256},
    utils::keccak256,
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

        let mut item_ids_vec: Vec<Token> = Vec::new();
        for id in &item_ids {
            let id: String = id.to_string();
            let u256: U256 = U256::from_dec_str(&id).unwrap();
            item_ids_vec.push(Token::Uint(u256));
        }
        token_vec.push(Token::Array(item_ids_vec));

        let mut uris_vec: Vec<Token> = Vec::new();
        for uri in &uris {
            uris_vec.push(Token::String(uri.to_string()));
        }
        token_vec.push(Token::Array(uris_vec));

        let encoded_message: Vec<u8> = encode(&token_vec);
        encoded_message
    }

    pub async fn create_signature(
        &self,
        message: &Vec<u8>,
        verify: Option<bool>,
    ) -> Result<Signature> {
        let digest = keccak256(message);
        let signature: Signature = self.wallet.sign_message(&digest).await?;

        if verify.unwrap_or(false) {
            println!("Produced signature {}", signature.to_string());
            self.verify_signature(digest.clone(), signature).unwrap();
        }

        Ok(signature)
    }

    pub fn verify_signature(&self, digest: [u8; 32], signature: Signature) -> Result<()> {
        signature
            .verify(digest.to_vec(), self.wallet.address())
            .unwrap();
        println!("Verified signature produced by {:?}", self.wallet.address());

        Ok(())
    }
}
