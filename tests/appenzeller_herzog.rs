use ris::parse_ris_nom;
use ris::RisParser;
use std::fs;

#[test]
fn parse_nom() {
    let file_path = "benches/files/Appenzeller-Herzog_2019.ris";
    let contents = fs::read_to_string(file_path).unwrap();
    let (_, output) = parse_ris_nom(&contents).unwrap();
    assert_eq!(output.len(), 3453)
}

#[test]
fn parse_handwritten() {
    let file_path = "benches/files/Appenzeller-Herzog_2019.ris";
    let contents = fs::read_to_string(file_path).unwrap();
    let parser = RisParser::default();
    let output = parser.parse(&contents).unwrap();
    assert_eq!(output.len(), 3453);
}