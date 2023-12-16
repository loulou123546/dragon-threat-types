use serde::{Serialize, Deserialize};

use crate::diagram_cell::{CellPosition, Cellsize, GenericAttrsText};

// TrustBoundaryBox alias : trust-boundary-box

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
#[allow(non_snake_case)]
pub struct DiagramCellTrustBoundaryBox {
    pub position: CellPosition,
    pub size: Cellsize,
    pub visible: Option<bool>,
    pub angle: Option<f64>,
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



///////////////////////////////////

use crate::{GenericDiagramCell, GenericCellBlock};

impl GenericDiagramCell for DiagramCellTrustBoundaryBox {
    fn get_id (&self) -> String {
        self.id.clone()
    }
    fn get_shape (&self) -> String {
        String::from("actor")
    }
    fn is_visible (&self) -> bool {
        match self.visible {
            Some(v) => v,
            None => true
        }
    }
    fn set_visible (&mut self, visibility: bool) {
        self.visible = Some(visibility);
    }
    fn get_angle (&self) -> f64 {
        match self.angle {
            Some(v) => v,
            None => 0.0
        }
    }
    fn set_angle (&mut self, angle: f64) {
        self.angle = Some(angle);
    }
}

impl GenericCellBlock for DiagramCellTrustBoundaryBox {
    fn get_position (&self) -> super::CellPosition {
        self.position.clone()
    }
    fn set_position (&mut self, position: super::CellPosition) {
        self.position = position;
    }
    fn get_size (&self) -> super::Cellsize {
        self.size.clone()
    }
    fn set_size (&mut self, size: super::Cellsize) {
        self.size = size;
    }
}
