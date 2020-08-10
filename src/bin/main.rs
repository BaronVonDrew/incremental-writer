extern crate incrementaljson;
use incrementaljson::incrementaljsonwriter;
use serde::{Serialize, Deserialize};

fn main() {
    let rows: Vec<Record> = vec![0, 1, 2, 3, 4, 5, 6, 7 , 8, 9, 10]
        .iter()
        .map(|num| Record { name: String::from("Test"), detail: *num})
        .collect();

    let out = std::fs::File::create("test.json").unwrap();

    let mut writer = incrementaljsonwriter::IncrementalJsonWriter::new(out);
    for row in rows {
        if row.detail == 8 { 
            panic!();
        }
        writer.write_json(&row).unwrap();
    }
}
#[derive(Serialize, Deserialize, Debug)]
struct Record { 
    name: String,
    detail: u32
}