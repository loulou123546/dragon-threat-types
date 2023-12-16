use serde::{Serialize, Deserialize};

use crate::diagram_cell::{CellPosition, CellId, LabelsVariant};

// TrustBoundaryCurve alias : trust-boundary-curve

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
#[allow(non_snake_case)]
pub struct DiagramCellTrustBoundaryCurve {
    pub source: DiagramCellTrustBoundaryCurvePosition,
    pub target: DiagramCellTrustBoundaryCurvePosition,
    pub width: f64,
    pub height: f64,
    pub connector: String,
    
    #[serde(default)]
    pub labels: Vec<LabelsVariant>,
    #[serde(default)]
    pub vertices: Vec<CellPosition>,

    pub id: String,
    pub zIndex: i64,
    pub data: CellDataTrustBoundaryCurve,
    pub attrs: Option<CellAttrsTrustBoundaryCurve>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
#[serde(untagged)]
pub enum DiagramCellTrustBoundaryCurvePosition {
    CellPosition(CellPosition),
    CellId(CellId),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
#[allow(non_snake_case)]
pub struct CellAttrsTrustBoundaryCurve {
    pub line: CellAttrsTrustBoundaryCurveBody
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
#[allow(non_snake_case)]
pub struct CellAttrsTrustBoundaryCurveBody {
    #[serde(default)]
    pub targetMarker: String,
    #[serde(default)]
    pub sourceMarker: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
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



///////////////////////////////////

use crate::{GenericDiagramCell};

impl GenericDiagramCell for DiagramCellTrustBoundaryCurve {
    fn get_id (&self) -> String {
        self.id.clone()
    }
    fn get_shape (&self) -> String {
        String::from("actor")
    }
    fn is_visible (&self) -> bool {
        true
    }
    fn set_visible (&mut self, visibility: bool) {}
    fn get_angle (&self) -> f64 {
        0.0
    }
    fn set_angle (&mut self, angle: f64) {}
}
