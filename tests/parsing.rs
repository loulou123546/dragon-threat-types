use threat_dragon_types;
use serde_json;

fn read_file (file: &str) -> threat_dragon_types::ThreatModelV2 {
    let raw = std::fs::read(file).expect(&format!("Failed to read file {file}"));
    let text = std::str::from_utf8(&raw).expect(&format!("Invalid UTF-8 sequence in file {file}"));
    serde_json::from_str(text).expect(&format!("Invalid JSON parsing into model for file {file}"))
}

#[test]
pub fn parse_summary () {
    let v2 = read_file("tests/demo_files/Demo_Threat_Model_V2.json");
    assert_eq!(v2.summary.title, Some("Demo Threat Model".to_owned()));
    assert_eq!(v2.summary.owner, Some("Mike Goodwin".to_owned()));
    assert_eq!(v2.summary.description, Some("A sample model of a web application, with a queue-decoupled background process.".to_owned()));
    assert_eq!(v2.summary.id, Some(0));
}
