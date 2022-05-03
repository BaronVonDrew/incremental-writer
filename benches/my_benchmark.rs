use criterion::{black_box, criterion_group, criterion_main, Criterion};
use incremental_writer::json::IncrementalJsonWriter;
use serde::{Deserialize, Serialize};
use std::ops::Range;

#[derive(Serialize, Deserialize, Debug)]
struct Record {
    name: String,
    detail: usize,
}

fn bench_incremental_write(c: &mut Criterion) {
    let path = "bench_incremental_write.json";
    let rows: Vec<Record> = (0..=3)
        .map(|num| Record {
            name: String::from("Test"),
            detail: num,
        })
        .collect();
    c.bench_function("bench_incremental_write", |b| {
        b.iter(|| write_vector_incrementally(&rows, path))
    });
}

fn bench_traditional_write(c: &mut Criterion) {
    let path = "bench_traditional_write.json";
    let rows: Vec<Vec<Record>> = (0..3)
        .fold(Vec::<Vec<usize>>::with_capacity(4), |mut current, next| {
            if next == 0 {
                current.resize(4, Vec::new());
                current[0] = vec![0];
            } else {
                let mut previous = current[next - 1].clone();
                previous.push(next);
                current[next] = previous;
            }

            current
        })
        .iter()
        .map(|vec| {
            vec.iter()
                .map(|num| Record {
                    name: String::from("Test"),
                    detail: *num,
                })
                .collect()
        })
        .collect();
    c.bench_function("bench_traditional_write", |b| {
        b.iter(|| write_vector_on_update(&rows, path))
    });
}

fn write_vector_incrementally(vector: &Vec<Record>, path: &str) {
    let file_buffer = std::fs::File::create(path).unwrap();
    let mut writer = IncrementalJsonWriter::new(file_buffer);

    for row in vector.iter() {
        writer.write_json(&row).unwrap();
    }
}

fn write_vector_on_update(vector: &Vec<Vec<Record>>, path: &str) {
    for row in vector.iter() {
        let json = serde_json::to_string_pretty(&row).unwrap();
        std::fs::write(path, json).unwrap();
    }
}

criterion_group!(benches, bench_incremental_write, bench_traditional_write);
criterion_main!(benches);
