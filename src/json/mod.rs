use serde::{Deserialize, Serialize};
use std::io::{Seek, SeekFrom, Write};

#[cfg(unix)]
use std::os::unix::fs::FileExt;
#[cfg(windows)]
use std::os::windows::fs::FileExt;

type Result<T> = std::result::Result<T, std::io::Error>;

pub struct IncrementalJsonWriter<T: FileExt + Write + Seek> {
    buffer: T,
}
impl<T: FileExt + Write + Seek> IncrementalJsonWriter<T> {
    pub fn new(buffer: T) -> Self {
        IncrementalJsonWriter::<T> { buffer }
    }

    pub fn write_json<U: Serialize>(&mut self, element: &U) -> Result<usize> {
        self.write(serde_json::to_string_pretty(&element)?.as_bytes())
    }
    #[cfg(unix)]
    fn write_at_offset(&mut self, bytes: &[u8], offset: u64) -> Result<usize> {
        let bytes_written = self.buffer.write_at(bytes, offset)?;
        self.buffer
            .seek(SeekFrom::Current((bytes_written - 2) as i64))
            .map(|_| bytes_written)
    }
    #[cfg(windows)]
    fn write_at_offset(&mut self, bytes: &[u8], offset: u64) -> Result<usize> {
        self.buffer.seek_write(bytes, offset)
    }
}

impl<T: FileExt + Write + Seek> Write for IncrementalJsonWriter<T> {
    fn write(&mut self, element: &[u8]) -> Result<usize> {
        let mut current = self.buffer.seek(SeekFrom::Current(0))?;
        let mut bytes = vec![];

        if current == 0 {
            self.buffer.write(b"[\n\n]")?;
            current = self.buffer.seek(SeekFrom::Current(0))?;
        } else {
            bytes.extend(b",\n");
        }

        bytes.extend(element);
        bytes.push(b'\n');
        bytes.push(b']');

        let written = self.write_at_offset(&bytes, current - 2)?;

        let new_position = self.buffer.seek(SeekFrom::Current(0))?;
        println!(
            "Was: {}, now: {}, total: {}",
            current,
            new_position,
            current + bytes.len() as u64
        );
        Ok(written)
    }
    fn flush(&mut self) -> Result<()> {
        self.buffer.flush()
    }
}

#[test]
fn writer_writes_square_brackets_to_buffer() {
    use std::io::Read;

    let expect_one = String::from(
        "[\n{\n  \"name\": \"Test\",\n  \"detail\": 0\n},\
    \n{\n  \"name\": \"Test\",\n  \"detail\": 1\n}\n]",
    );

    let expect_two = String::from(
        "[\n{\n  \"name\": \"Test\",\n  \"detail\": 0\n},\
    \n{\n  \"name\": \"Test\",\n  \"detail\": 1\n},\
    \n{\n  \"name\": \"Test\",\n  \"detail\": 2\n},\
    \n{\n  \"name\": \"Test\",\n  \"detail\": 3\n}\n]",
    );

    let path = "unittest.json";
    let rows: Vec<Record> = vec![0, 1, 2, 3]
        .iter()
        .map(|num| Record {
            name: String::from("Test"),
            detail: *num,
        })
        .collect();
    let writer = std::fs::File::create(path).unwrap();
    let mut json_writer = IncrementalJsonWriter::new(writer);

    let mut reader = std::fs::File::open(path).unwrap();

    for row in rows.iter().take(2) {
        json_writer.write_json(&row).unwrap();
    }
    let mut buffer = String::new();
    reader.read_to_string(&mut buffer).unwrap();
    assert_eq!(expect_one, buffer);

    for row in rows.iter().skip(2).take(2) {
        json_writer.write_json(&row).unwrap();
    }
    let mut buffer = String::new();
    reader.seek(SeekFrom::Start(0)).unwrap();
    reader.read_to_string(&mut buffer).unwrap();
    assert_eq!(expect_two, buffer);
}

#[derive(Serialize, Deserialize, Debug)]
struct Record {
    name: String,
    detail: u32,
}
