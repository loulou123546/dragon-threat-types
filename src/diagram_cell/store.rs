use serde::{Serialize, Deserialize};

use crate::diagram_cell::{CellPosition, Cellsize, Threat, GenericAttrsText, GenericAttrsBorder};

// Store alias : store

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
#[allow(non_snake_case)]
pub struct DiagramCellStore {
    pub position: CellPosition,
    pub size: Cellsize,
    pub visible: Option<bool>,
    pub angle: Option<f64>,
    pub id: String,
    pub zIndex: i64,
    pub attrs: CellAttrsStore,
    pub data: CellDataStore
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
#[allow(non_snake_case)]
pub struct CellAttrsStore {
    pub text: Option<GenericAttrsText>,
    pub topLine: GenericAttrsBorder,
    pub bottomLine: GenericAttrsBorder
}


#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
#[allow(non_snake_case)]
pub struct CellDataStore {
    pub r#type: String,
    pub name: String,
    
    #[serde(default)]
    pub description: String,
    
    #[serde(default)]
    pub outOfScope: bool,
    
    #[serde(default)]
    pub reasonOutOfScope: String,
    
    #[serde(default)]
    pub isALog: bool,
    
    #[serde(default)]
    pub storesCredentials: bool,
    
    #[serde(default)]
    pub isEncrypted: bool,
    
    #[serde(default)]
    pub isSigned: bool,
    
    #[serde(default)]
    pub isTrustBoundary: bool,
    
    #[serde(default)]
    pub hasOpenThreats: bool,
    
    #[serde(default)]
    pub threats: Vec<Threat>
}
