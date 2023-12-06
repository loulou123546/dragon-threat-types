use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ThreatModelV2 {
    pub summary: Summary
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Summary {
    pub title: Option<String>,
    pub owner: Option<String>,
    pub description: Option<String>,
    pub id: Option<usize>
}
