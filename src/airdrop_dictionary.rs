use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufWriter, Write};

use bincode::serialize_into;
use calamine::{open_workbook, Error, Reader, Xlsx};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct AirdropDictionary {
    hash_map: HashMap<String, i32>,
}

impl AirdropDictionary {
    pub fn read_file_excel_and_save() -> Result<(), Error> {
        let path = format!("{}/excels/NFTAirdrop.xlsx", env!("CARGO_MANIFEST_DIR"));
        let mut excel: Xlsx<_> = open_workbook(path)?;

        let mut dictionary = AirdropDictionary {
            hash_map: HashMap::with_capacity(5000),
        };

        if let Some(Ok(r)) = excel.worksheet_range("Sheet1") {
            let mut rows = r.rows();
            rows.next();

            for row in rows {
                let counter = dictionary
                    .hash_map
                    .entry(row[1].to_string().to_lowercase())
                    .or_insert(0);
                *counter += 1;
            }
        }

        dictionary.hash_map.insert("0xBd57D3A669147F88166A88765e0BFb493ae00c5B".to_string().to_lowercase(), 2);

        dictionary.save();

        Ok(())
    }

    pub fn load() -> Self {
        let file = File::open("airdrop").unwrap();
        let dictionary: AirdropDictionary =
            bincode::deserialize_from(BufReader::new(file)).unwrap();
        return dictionary;
    }

    pub fn save(&self) {
        let mut writer = BufWriter::new(File::create("airdrop").unwrap());
        serialize_into(&mut writer, &self).unwrap();
        writer.flush().unwrap();
    }

    pub fn get(&self, address: &str) -> Option<i32> {
        match self.hash_map.get(&address.to_lowercase()) {
            Some(value) => Some(*value),
            None => None
        }
    }
}
