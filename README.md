# IncrementalJsonWriter

A rolling file writer which serialises elements to JSON using [serde](https://github.com/serde-rs/json) and appends them to an array in a file. 

Currently a work in progress and only supports Windows but linux support coming very soon. 


IncrementalJsonWriter provides a writer that takes a File writer
and incrementally writes JSON objects to an array inside that file using serde_json

It implements the write trait's `write(&[u8])` and `flush()` as well as `write_json()` which takes
any serialisable object and writes it to the underlying array.

example: 

    use incrementaljsonwriter::IncrementalJsonWriter;
    use serde::{Serialize, Deserialize};
    fn main() {
        let rows: Vec<Record> = vec![0, 1, 2, 3, 4, 5, 6, 7 , 8, 9, 10]
            .iter()
            .map(|num| Record { name: String::from("Test"), detail: *num})
            .collect();
    let out = std::fs::File::create("test.json").unwrap();
        let mut writer = IncrementalJsonWriter::new(out);
        for row in rows {
            if row.detail == 8 { 
                //even though the program panics here, the file output is still
                //a valid JSON array. 
                panic!();
            }
            //&row is serialised here and written to an array in a file
            writer.write_json(&row).unwrap();
        }
    }
    #[derive(Serialize, Deserialize, Debug)]
    struct Record { 
        name: String,
        detail: u32
    }
