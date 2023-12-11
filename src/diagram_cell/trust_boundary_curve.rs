use serde::{Serialize, Deserialize};

use crate::diagram_cell::{CellPosition, CellId};

// TrustBoundaryCurve alias : trust-boundary-curve

#[derive(Serialize, Deserialize, Debug, Clone)]
#[allow(non_snake_case)]
pub struct DiagramCellTrustBoundaryCurve {
    pub source: DiagramCellTrustBoundaryCurvePosition,
    pub target: DiagramCellTrustBoundaryCurvePosition,
    pub width: f64,
    pub height: f64,
    pub connector: String,
    pub id: String,
    pub zIndex: i64,
    pub data: CellDataTrustBoundaryCurve
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum DiagramCellTrustBoundaryCurvePosition {
    CellPosition(CellPosition),
    CellId(CellId),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[allow(non_snake_case)]
pub struct CellDataTrustBoundaryCurve {
    pub r#type: String,
    pub name: String,
    
    #[serde(default)]
    pub description: String,
    
    #[serde(default)]
    pub isTrustBoundary: bool,
    
    #[serde(default)]
    pub hasOpenThreats: bool
}
