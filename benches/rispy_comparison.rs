use criterion::{criterion_group, criterion_main, Criterion};
use ris::*;
use std::fs;

pub fn parse_file(file_path: &str) {
    let contents = fs::read_to_string(file_path).unwrap();
    parse_ris_nom(&contents).unwrap();
}

pub fn appenzeller_herzog(c: &mut Criterion) {
    let file_path = "benches/files/Appenzeller-Herzog_2019.ris";
    c.bench_function("appenzeller_herzog", |b| b.iter(|| parse_file(&file_path)));
}

pub fn kwok(c: &mut Criterion) {
    let file_path = "benches/files/Kwok_2020.ris";
    c.bench_function("kwok", |b| b.iter(|| parse_file(&file_path)));
}

criterion_group!(benches, appenzeller_herzog, kwok);
criterion_main!(benches);
