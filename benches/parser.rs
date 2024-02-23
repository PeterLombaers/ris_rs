use criterion::{criterion_group, criterion_main, Criterion};
use ris::*;
use std::fs;


pub fn appenzeller_herzog_handwritten(c: &mut Criterion) {
    let file_path = "benches/files/Appenzeller-Herzog_2019.ris";
    let contents = fs::read(file_path).unwrap();
    let parser = RisParser::default();
    c.bench_function("appenzeller_herzog_handwritten", |b| b.iter(|| parser.parse(&contents)));
}

// pub fn ah_100_000_handwritten(c: &mut Criterion) {
//     let file_path = "benches/files/AH_100_000.ris";
//     let contents = fs::read_to_string(file_path).unwrap();
//     let parser = RisParser::default();
//     c.bench_function("ah_100_000_handwritten", |b| b.iter(|| parser.parse(&contents)));
// }

criterion_group!(
    benches,
    appenzeller_herzog_handwritten,
    // ah_100_000_handwritten,
);
criterion_main!(benches);
