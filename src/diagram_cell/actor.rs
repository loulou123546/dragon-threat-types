use serde::{Serialize, Deserialize};

use crate::diagram_cell::{CellPosition, Cellsize, Threat};

// Actor alias : actor

#[derive(Serialize, Deserialize, Debug, Clone)]
#[allow(non_snake_case)]
pub struct DiagramCellActor {
    pub position: CellPosition,
    pub size: Cellsize,
    pub visible: Option<bool>,
    pub angle: Option<f64>,
    pub id: String,
    pub zIndex: i64,
    pub attrs: CellAttrsActor,
    pub data: CellDataActor
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CellAttrsActor {
    pub body: CellAttrsActorBody
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[allow(non_snake_case)]
pub struct CellAttrsActorBody {
    pub stroke: String,
    pub strokeWidth: f64,
    pub strokeDasharray: Option<String>
}


#[derive(Serialize, Deserialize, Debug, Clone)]
#[allow(non_snake_case)]
pub struct CellDataActor {
    pub r#type: String,
    pub name: String,
    
    #[serde(default)]
    pub description: String,

    #[serde(default)]
    pub outOfScope: bool,
    
    #[serde(default)]
    pub reasonOutOfScope: String,
    
    #[serde(default)]
    pub providesAuthentication: bool,
    
    #[serde(default)]
    pub hasOpenThreats: bool,
    
    #[serde(default)]
    pub threats: Vec<Threat>
}
