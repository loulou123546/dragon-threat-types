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



///////////////////////////////////

use crate::{GenericDiagramCell, GenericCellBlock};

impl GenericDiagramCell for DiagramCellStore {
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

impl GenericCellBlock for DiagramCellStore {
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
