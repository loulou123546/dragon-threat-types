use serde::{Serialize, Deserialize};

use crate::diagram_cell::{CellPosition, Cellsize, Threat, GenericAttrsText, GenericAttrsBorder};

// Process alias : process

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
#[allow(non_snake_case)]
pub struct DiagramCellProcess {
    pub position: CellPosition,
    pub size: Cellsize,
    pub visible: Option<bool>,
    pub angle: Option<f64>,
    pub id: String,
    pub zIndex: i64,
    pub attrs: CellAttrsProcess,
    pub data: CellDataProcess
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct CellAttrsProcess {
    pub body: Option<GenericAttrsBorder>,
    pub text: Option<GenericAttrsText>
}


#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
#[allow(non_snake_case)]
pub struct CellDataProcess {
    pub r#type: String,
    pub name: String,
    
    #[serde(default)]
    pub description: String,

    #[serde(default)]
    pub outOfScope: bool,

    #[serde(default)]
    pub reasonOutOfScope: String,
    
    #[serde(default)]
    pub isTrustBoundary: bool,

    #[serde(default)]
    pub hasOpenThreats: bool,

    #[serde(default)]
    pub privilegeLevel: String,

    #[serde(default)]
    pub threats: Vec<Threat>
}
