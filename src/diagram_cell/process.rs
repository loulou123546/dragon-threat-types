use serde::{Serialize, Deserialize};

use crate::diagram_cell::{CellPosition, Cellsize, Threat};

// Process alias : process

#[derive(Serialize, Deserialize, Debug, Clone)]
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
pub struct CellAttrsProcess {
    pub body: CellAttrsProcessBody
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[allow(non_snake_case)]
pub struct CellAttrsProcessBody {
    pub stroke: String,
    pub strokeWidth: f64,
    pub strokeDasharray: Option<String>
}


#[derive(Serialize, Deserialize, Debug, Clone)]
#[allow(non_snake_case)]
pub struct CellDataProcess {
    pub r#type: String,
    pub name: String,

    #[serde(default)]
    pub outOfScope: bool,

    #[serde(default)]
    pub reasonOutOfScope: String,

    #[serde(default)]
    pub hasOpenThreats: bool,

    #[serde(default)]
    pub privilegeLevel: String,

    #[serde(default)]
    pub threats: Vec<Threat>
}
