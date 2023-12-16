use serde::{Serialize, Deserialize};

use crate::diagram_cell::{CellPosition, Cellsize, GenericAttrsText};

// TrustBoundaryBox alias : trust-boundary-box

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
#[allow(non_snake_case)]
pub struct DiagramCellTrustBoundaryBox {
    pub position: CellPosition,
    pub size: Cellsize,
    pub id: String,
    pub zIndex: i64,
    pub data: CellDataTrustBoundaryBox,
    pub attrs: Option<CellAttrsTrustBoundaryBox>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
#[allow(non_snake_case)]
pub struct CellAttrsTrustBoundaryBox {
    pub headerText: GenericAttrsText
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
#[allow(non_snake_case)]
pub struct CellDataTrustBoundaryBox {
    pub r#type: String,
    pub name: String,
    
    #[serde(default)]
    pub description: String,
    
    #[serde(default)]
    pub isTrustBoundary: bool,
    
    #[serde(default)]
    pub hasOpenThreats: bool
}
