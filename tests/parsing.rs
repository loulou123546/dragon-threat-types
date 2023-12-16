use threat_dragon_types;
use serde_json;
use std::fs;

fn read_file (file: &str) -> threat_dragon_types::ThreatModelV2 {
    let raw = std::fs::read(file).expect(&format!("Failed to read file {file}"));
    let text = std::str::from_utf8(&raw).expect(&format!("Invalid UTF-8 sequence in file {file}"));
    serde_json::from_str(text).expect(&format!("Invalid JSON parsing into model for file {file}"))
}

#[test]
pub fn parse_empty () {
    let v2 = read_file("tests/demo_files/EmptyModel.json");

    assert_eq!(v2.version, "2.1.2".to_owned());
    
    let _ = fs::write("output.json", format!("{}", serde_json::to_string(&v2).expect("Error serializing")));
}

#[test]
pub fn parse_root () {
    let v2 = read_file("tests/demo_files/Demo_Threat_Model_V2.json");

    assert_eq!(v2.version, "2.1.2".to_owned());
}

#[test]
pub fn parse_summary () {
    let v2 = read_file("tests/demo_files/Demo_Threat_Model_V2.json");

    assert_eq!(v2.summary.title, Some("Demo Threat Model".to_owned()));
    assert_eq!(v2.summary.owner, Some("Mike Goodwin".to_owned()));
    assert_eq!(v2.summary.description, Some("A sample model of a web application, with a queue-decoupled background process.".to_owned()));
    assert_eq!(v2.summary.id, Some(0));
}

#[test]
pub fn parse_details () {
    let v2 = read_file("tests/demo_files/Demo_Threat_Model_V2.json");

    match v2.detail.contributors {
        Some(contribs) => {
            assert!(contribs.get(0).is_some_and(|x| x.name == "Tom Brown"));
            assert!(contribs.get(1).is_some_and(|x| x.name == "Albert Moneypenny"));
        }
        None => panic!("Contributors is None while expected to be Tom Brown + Albert Moneypenny")
    };
    assert_eq!(v2.detail.reviewer, Some("Jane Smith".to_owned()));
    assert_eq!(v2.detail.threatTop, Some(0));
    assert_eq!(v2.detail.diagramTop, Some(0));
    assert!(v2.detail.diagrams.is_some());
}

#[test]
pub fn parse_diagram_info () {
    let v2 = read_file("tests/demo_files/Demo_Threat_Model_V2.json");
    let diagrams = v2.detail.diagrams.expect("Found no diagrams");
    let diagram = diagrams.get(0).expect("Diagram at index 0 does not exist");

    assert_eq!(diagram.version, Some("2.0".to_owned()));
    assert_eq!(diagram.title, Some("Main Request Data Flow".to_owned()));
    assert_eq!(diagram.description, Some("Main Request Data Flow Description".to_owned()));
    assert_eq!(diagram.thumbnail, Some("./public/content/images/thumbnail.stride.jpg".to_owned()));
    assert_eq!(diagram.diagramType, "STRIDE".to_owned());
    assert_eq!(diagram.id, Some(0));
}
