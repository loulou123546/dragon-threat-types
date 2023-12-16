use serde::{Serialize, Deserialize};

pub mod process;
pub mod store;
pub mod actor;
pub mod td_text_block;
pub mod flow;
pub mod trust_boundary_curve;
pub mod trust_boundary_box;


#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct CellPosition {
    pub x: f64,
    pub y: f64
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct Cellsize {
    pub width: f64,
    pub height: f64
}


#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
#[allow(non_snake_case)]
pub struct Threat {
    pub id: String,
    
    #[serde(default)]
    pub title: String,
    
    #[serde(default)]
    pub status: String,
    
    #[serde(default)]
    pub severity: String,
    
    #[serde(default)]
    pub r#type: String,
    
    #[serde(default)]
    pub description: String,
    
    #[serde(default)]
    pub mitigation: String,
    
    #[serde(default)]
    pub modelType: String,
    
    #[serde(default)]
    pub new: bool,
    
    #[serde(default)]
    pub number: u64,

    #[serde(default)]
    pub score: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct CellId {
    pub cell: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct GenericAttrsText {
    #[serde(default)]
    pub text: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
#[allow(non_snake_case)]
pub struct GenericAttrsBorder {
    pub stroke: Option<String>,
    pub strokeWidth: f64,
    pub strokeDasharray: Option<String>
}


#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
#[serde(untagged)]
pub enum LabelsVariant {
    Simple(String),
    Older { position: Option<f64>, attrs: OlderLabelAttrs } 
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct OlderLabelAttrs {
    #[serde(alias = "text")]
    pub label : GenericAttrsText
}
