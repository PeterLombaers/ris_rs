use ris::RisParser;
use std::fs::File;
use std::io::{self, BufRead};
use std::fs;

#[test]
fn parse_handwritten() {
    let csv_file_path = "benches/files/Appenzeller-Herzog_2019.csv";
    let ris_file_path = "benches/files/Appenzeller-Herzog_2019.ris";

    let file = File::open(csv_file_path).unwrap();
    let reader = io::BufReader::new(file);

    let num_lines = reader.lines().count() - 1;

    let contents = fs::read(ris_file_path).unwrap();
    let parser = RisParser::default();
    let output = parser.parse(&contents).unwrap();

    assert_eq!(output.len(), num_lines);
}
