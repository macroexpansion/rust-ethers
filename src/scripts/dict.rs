use airdrop_cmc::airdrop_dictionary::AirdropDictionary;

fn main() {
    AirdropDictionary::read_file_excel_and_save().unwrap();

    let dictionary = AirdropDictionary::load();
    let res = dictionary.get("0xBd57D3A669147F88166A88765e0BFb493ae00c5B");
    println!("{}", res.unwrap());
}
