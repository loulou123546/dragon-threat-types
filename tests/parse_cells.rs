use threat_dragon_types::{self, DiagramCell};
use serde_json;

fn read_file (file: &str) -> threat_dragon_types::ThreatModelV2 {
    let raw = std::fs::read(file).expect(&format!("Failed to read file {file}"));
    let text = std::str::from_utf8(&raw).expect(&format!("Invalid UTF-8 sequence in file {file}"));
    serde_json::from_str(text).expect(&format!("Invalid JSON parsing into model for file {file}"))
}

fn get_exemple_diagram () -> threat_dragon_types::Diagram {
    let v2 = read_file("tests/demo_files/Full_featured_model.json");
    let diagrams = v2.detail.diagrams.expect("Found no diagrams");
    diagrams[0].clone()
}

#[test]
pub fn check_store () {
    let diagram = get_exemple_diagram();

    let cell = diagram.find_cell_id("a6712fb0-c1a2-4294-aa7e-44eea7ce49e3").expect("Cannot found store");
    let cell = match cell {
        DiagramCell::Store(el) => Some(el),
        _ => None
    }.expect("Element with this id is not a Store");

    assert_eq!(cell.data.hasOpenThreats, false);
    let threat = cell.data.threats[0].clone();

    assert_eq!(threat.title, "Single threat".to_owned());
    assert_eq!(threat.status, "Mitigated".to_owned());
    assert_eq!(threat.severity, "Medium".to_owned());
    assert_eq!(threat.r#type, "Integrity".to_owned());
    assert_eq!(threat.description, "Fake".to_owned());
    assert_eq!(threat.mitigation, "OK".to_owned());
    assert_eq!(threat.modelType, "Generic".to_owned());
    assert_eq!(threat.new, false);
    assert_eq!(threat.number, 7);
    assert_eq!(threat.score, "2".to_owned());
}
