use serde::{Serialize, Deserialize};

use crate::diagram_cell::{CellPosition, Cellsize, GenericAttrsText};

// TdTextBlock alias : td-text-block

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
#[allow(non_snake_case)]
pub struct DiagramCellTextBox {
    pub position: CellPosition,
    pub size: Cellsize,
    pub visible: Option<bool>,
    pub angle: Option<f64>,
    pub id: String,
    pub zIndex: i64,
    pub data: CellDataTextBox,
    pub attrs: Option<CellAttrsTextBox>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct CellAttrsTextBox {
    pub text: GenericAttrsText
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
#[allow(non_snake_case)]
pub struct CellDataTextBox {
    pub r#type: String,
    pub name: String,
    
    #[serde(default)]
    pub isTrustBoundary: bool,
    
    #[serde(default)]
    pub hasOpenThreats: bool
}
