use ris::parse_ris;
use std::fs;


#[test]
fn parse() {
    let file_path = "benches/files/Appenzeller-Herzog_2019.ris";
    let contents = fs::read_to_string(file_path).unwrap();
    let (_, output) = parse_ris(&contents).unwrap();
    assert_eq!(output.len(), 3453)
}
