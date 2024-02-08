use criterion::{criterion_group, criterion_main, Criterion};
use ris::*;

pub fn parse_reference(c: &mut Criterion) {
    let reference_string = "TY  - JOUR
ID  - 12345
T1  - Title of reference
A1  - Marx, Karl
A1  - Lindgren, Astrid
A2  - Glattauer, Daniel
Y1  - 2014//
N2  - BACKGROUND: Lorem ipsum dolor sit amet, consectetuer adipiscing elit. Aenean commodo ligula eget dolor. Aenean massa. Cum sociis natoque penatibus et magnis dis parturient montes, nascetur ridiculus mus.  RESULTS: Donec quam felis, ultricies nec, pellentesque eu, pretium quis, sem. Nulla consequat massa quis enim. CONCLUSIONS: Donec pede justo, fringilla vel, aliquet nec, vulputate eget, arcu. In enim justo, rhoncus ut, imperdiet a, venenatis vitae, justo. Nullam dictum felis eu pede mollis pretium.
KW  - Pippi
KW  - Nordwind
KW  - Piraten
JF  - Lorem
JA  - lorem
VL  - 9
IS  - 3
SP  - e0815
CY  - United States
PB  - Fun Factory
SN  - 1932-6208
M1  - 1008150341
L2  - http://example.com
UR  - http://example_url.com
ER  - ";
    let parser = RisParser::default();
    c.bench_function("parse_reference", |b| {
        b.iter(|| parser.parse(&reference_string))
    });
}

criterion_group!(
    benches,
    parse_reference,
);
criterion_main!(benches);
