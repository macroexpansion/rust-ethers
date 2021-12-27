use ethers_rust::airdrop_dictionary::AirdropDictionary;
use ethers_rust::signing::MessageSigner;


fn look() {
    let dictionary = AirdropDictionary::load();
    dictionary.get("0x731D01a3553079628A6b2C7CB1F22cF0617290ad");
    dictionary.get("0xe87C194C70A2b9DA81112037F6586Dc206ae28fE");
}

fn sign_message() {
    let address = "0xe03d8e97fb3bea6a550427c2841660e69ede053e";
    let item_ids = vec![10,11,12];

    let signer = MessageSigner::from_env("PRIVATE_KEY");
    let message = MessageSigner::encode_message(address, item_ids);
    let signature = signer.create_signature(&message).unwrap();
    signer.verify_signature(message, signature).unwrap();
}

fn main() {
    
}
