mod incrementaljsonwriter;
use serde::ser::{Serializer, SerializeSeq};
use serde::{Serialize, Deserialize};

fn main() {
    let rows: Vec<Record> = vec![0, 1, 2, 3, 4, 5, 6, 7 , 8, 9, 10]
        .iter()
        .map(|num| Record { name: String::from("Test"), detail: *num})
        .collect();

    let out = std::io::stdout();

    let mut ser = serde_json::Serializer::new(out);
    let mut seq = ser.serialize_seq(Some(rows.len())).unwrap(); // or None if unknown
    for row in rows {
        if row.detail == 8 { 
            panic!();
        }
        seq.serialize_element(&row).unwrap();
    }
    seq.end().unwrap();
}
#[derive(Serialize, Deserialize, Debug)]
struct Record { 
    name: String,
    detail: u32
}