use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ThreatModelV2 {
    pub summary: Summary,
    pub detail: Detail,
    pub version: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Summary {
    pub title: Option<String>,
    pub owner: Option<String>,
    pub description: Option<String>,
    pub id: Option<usize>
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct Detail {
    pub contributors: Option<Vec<Contributor>>,
    pub reviewer: Option<String>,
    pub diagramTop: Option<usize>,
    pub threatTop: Option<usize>,
    pub diagrams: Option<Vec<Diagram>>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Contributor {
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct Diagram {
    // pub cells
    pub version: Option<String>,
    pub title: Option<String>,
    pub descrition: Option<String>,
    pub thumbnail: Option<String>,
    pub diagramType: String,
    pub id: Option<usize>
}
