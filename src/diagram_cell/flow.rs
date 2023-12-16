use serde::{Serialize, Deserialize};

use crate::diagram_cell::{CellPosition, Threat, CellId, LabelsVariant};

// Flow alias : flow

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
#[allow(non_snake_case)]
pub struct DiagramCellFlow {
    pub source: DiagramCellFlowPosition,
    pub target: DiagramCellFlowPosition,
    pub width: f64,
    pub height: f64,
    pub connector: String,
    
    #[serde(default)]
    pub labels: Vec<LabelsVariant>,
    #[serde(default)]
    pub vertices: Vec<CellPosition>,

    pub id: String,
    pub zIndex: i64,
    pub attrs: CellAttrsFlow,
    pub data: CellDataFlow
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
#[serde(untagged)]
pub enum DiagramCellFlowPosition {
    CellPosition(CellPosition),
    CellId(CellId),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
#[allow(non_snake_case)]
pub struct CellAttrsFlow {
    pub line: CellAttrsFlowBody
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
#[allow(non_snake_case)]
pub struct CellAttrsFlowBody {
    pub stroke: String,
    pub strokeWidth: Option<f64>,
    pub targetMarker: Option<CellAttrsFlowMarker>,
    pub sourceMarker: Option<CellAttrsFlowMarker>,
    pub strokeDasharray: Option<String>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct CellAttrsFlowMarker {
    pub name: String
}


#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
#[allow(non_snake_case)]
pub struct CellDataFlow {
    pub r#type: String,
    pub name: String,
    
    #[serde(default)]
    pub description: String,
    
    #[serde(default)]
    pub outOfScope: bool,
    
    #[serde(default)]
    pub reasonOutOfScope: String,
    
    #[serde(default)]
    pub isBidirectional: bool,
    
    #[serde(default)]
    pub isEncrypted: bool,
    
    #[serde(default)]
    pub isPublicNetwork: bool,
    
    #[serde(default)]
    pub protocol: String,
    
    #[serde(default)]
    pub isTrustBoundary: bool,
    
    #[serde(default)]
    pub hasOpenThreats: bool,
    
    #[serde(default)]
    pub threats: Vec<Threat>
}
