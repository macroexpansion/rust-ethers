use std::fs::File;
use std::io::{BufWriter, BufReader, Write};
use std::collections::HashMap;

use calamine::{open_workbook, Error, Xlsx, Reader};
use bincode::serialize_into;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct AirdropDictionary {
    hash_map: HashMap<String, i32>,
}

fn read_file_excel() -> Result<(), Error> {
    let path = format!("{}/excels/NFTAirdrop.xlsx", env!("CARGO_MANIFEST_DIR"));
    let mut excel: Xlsx<_> = open_workbook(path)?;

    let mut dictionary = AirdropDictionary {
        hash_map: HashMap::with_capacity(5000)
    };

    if let Some(Ok(r)) = excel.worksheet_range("Sheet1") {
        let mut rows = r.rows();
        rows.next();

        for row in rows {
            let counter = dictionary.hash_map.entry(row[1].to_string().to_lowercase()).or_insert(0);
            *counter += 1;
        }
    }

    lookup(&dictionary.hash_map, "0x731D01a3553079628A6b2C7CB1F22cF0617290ad");
    lookup(&dictionary.hash_map, "0xe87C194C70A2b9DA81112037F6586Dc206ae28fE");

    Ok(())
}

fn load() {
    let file = File::open("airdrop").unwrap();
    let dictionary: AirdropDictionary = bincode::deserialize_from(BufReader::new(file)).unwrap();
}

fn save() {
    let mut writer = BufWriter::new(File::create("airdrop").unwrap());
    bincode::serialize_into(&mut writer, &dictionary).unwrap();
    writer.flush().unwrap();
}

fn lookup(hash_map: &HashMap<String, i32>, address: &str) {
    match hash_map.get(&address.to_lowercase()) {
        Some(value) => {
            println!("do something here {}", value);
        },
        None => ()
    }
}

fn main() {
    match read_file_excel() {
        Ok(_ok) => {},
        Err(err) => println!("{}", err)
    };

    // load();
}
