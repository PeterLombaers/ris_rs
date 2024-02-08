use ris::RisParser;
use std::fs;

#[test]
fn parse_handwritten() {
    let file_path = "benches/files/Appenzeller-Herzog_2019.ris";
    let contents = fs::read_to_string(file_path).unwrap();
    let parser = RisParser::default();
    let output = parser.parse(&contents).unwrap();
    assert_eq!(output.len(), 3453);
}