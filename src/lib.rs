//! # incremental-writer
//!
//! incremental-writer is a library which provides writers to write in different formats to files on disk, currently
//! only JSON is supported. incremental_writer::json::IncrementalJsonWriter provides a writer that takes a File writer
//! and incrementally writes JSON objects to an array inside that file using serde_json.
//!
//! Works on both Windows and Unix.
//!
//! It implements the write trait's write(&[u8]) and flush() as well as write_json() which takes
//! any serialisable object and writes it to the underlying array.
//!
//! example:
//! ```
//! extern crate incremental_writer;
//! use incremental_writer::json;
//! use serde::{Serialize, Deserialize};
//!
//! fn main() {
//!     let rows: Vec<Record> = vec![0, 1, 2, 3, 4, 5, 6, 7 , 8, 9, 10]
//!         .iter()
//!         .map(|num| Record { name: String::from("Test"), detail: *num})
//!         .collect();
//!
//!     let out = std::fs::File::create("docstest.json").unwrap();
//!
//!     let mut writer = json::IncrementalJsonWriter::new(out);
//!     for row in rows {
//!         //the element is written to the file on each iteration
//!         //if it stops before finishing, the JSON is still valid
//!         writer.write_json(&row).unwrap();
//!     }
//! }
//! #[derive(Serialize, Deserialize, Debug)]
//! struct Record {
//!     name: String,
//!     detail: u32
//! }
//! ```
#![doc(html_root_url = "https://docs.rs/incremental-writer/0.1.2")]
pub mod json;
