use serde::{Serialize, Deserialize};

pub mod diagram_cell;
pub use diagram_cell::process::DiagramCellProcess;
pub use diagram_cell::actor::DiagramCellActor;
pub use diagram_cell::store::DiagramCellStore;
pub use diagram_cell::td_text_block::DiagramCellTextBox;
pub use diagram_cell::flow::DiagramCellFlow;
pub use diagram_cell::trust_boundary_curve::DiagramCellTrustBoundaryCurve;
pub use diagram_cell::trust_boundary_box::DiagramCellTrustBoundaryBox;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct ThreatModelV2 {
    pub summary: Summary,
    pub detail: Detail,
    pub version: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct Summary {
    pub title: Option<String>,
    pub owner: Option<String>,
    pub description: Option<String>,
    pub id: Option<usize>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
#[allow(non_snake_case)]
pub struct Detail {
    pub contributors: Option<Vec<Contributor>>,
    pub reviewer: Option<String>,
    pub diagramTop: Option<usize>,
    pub threatTop: Option<usize>,
    pub diagrams: Option<Vec<Diagram>>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct Contributor {
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
#[allow(non_snake_case)]
pub struct Diagram {
    pub cells: Vec<DiagramCell>,
    pub version: Option<String>,
    pub title: Option<String>,
    #[serde(alias = "descrition")]
    pub description: Option<String>,
    pub placeholder: Option<String>,
    pub thumbnail: Option<String>,
    pub diagramType: String,
    pub format: Option<String>,
    pub id: Option<usize>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "shape")]
pub enum DiagramCell {
    #[serde(rename(serialize = "process", deserialize = "process"))]
    Process(DiagramCellProcess),

    #[serde(rename(serialize = "store", deserialize = "store"))]
    Store(DiagramCellStore),

    #[serde(rename(serialize = "actor", deserialize = "actor"))]
    Actor(DiagramCellActor),

    #[serde(rename(serialize = "flow", deserialize = "flow"))]
    Flow(DiagramCellFlow),

    #[serde(rename(serialize = "trust-boundary-box", deserialize = "trust-boundary-box"))]
    TrustBoundaryBox(DiagramCellTrustBoundaryBox),

    #[serde(rename(serialize = "trust-boundary-curve", deserialize = "trust-boundary-curve"))]
    TrustBoundaryCurve(DiagramCellTrustBoundaryCurve),

    #[serde(rename(serialize = "td-text-block", deserialize = "td-text-block"))]
    TdTextBlock(DiagramCellTextBox)
}



//////////////////////////////

pub trait GenericDiagramCell {
    fn get_id (&self) -> String;
    fn get_shape (&self) -> String;
    fn is_visible (&self) -> bool;
    fn set_visible (&mut self, visibility: bool);
    fn get_angle (&self) -> f64;
    fn set_angle (&mut self, angle: f64);
}

pub trait GenericCellBlock {
    fn get_position (&self) -> diagram_cell::CellPosition;
    fn set_position (&mut self, position: diagram_cell::CellPosition);
    fn get_size (&self) -> diagram_cell::Cellsize;
    fn set_size (&mut self, size: diagram_cell::Cellsize);
}

impl GenericDiagramCell for DiagramCell {
    fn get_id (&self) -> String {
        match self {
            DiagramCell::Actor(a) => a.get_id(),
            DiagramCell::Flow(a) => a.get_id(),
            DiagramCell::Store(a) => a.get_id(),
            DiagramCell::Process(a) => a.get_id(),
            DiagramCell::TrustBoundaryBox(a) => a.get_id(),
            DiagramCell::TrustBoundaryCurve(a) => a.get_id(),
            DiagramCell::TdTextBlock(a) => a.get_id()
        }
    }
    fn get_angle (&self) -> f64 {
        match self {
            DiagramCell::Actor(a) => a.get_angle(),
            DiagramCell::Flow(a) => a.get_angle(),
            DiagramCell::Store(a) => a.get_angle(),
            DiagramCell::Process(a) => a.get_angle(),
            DiagramCell::TrustBoundaryBox(a) => a.get_angle(),
            DiagramCell::TrustBoundaryCurve(a) => a.get_angle(),
            DiagramCell::TdTextBlock(a) => a.get_angle()
        }
    }
    fn is_visible (&self) -> bool {
        match self {
            DiagramCell::Actor(a) => a.is_visible(),
            DiagramCell::Flow(a) => a.is_visible(),
            DiagramCell::Store(a) => a.is_visible(),
            DiagramCell::Process(a) => a.is_visible(),
            DiagramCell::TrustBoundaryBox(a) => a.is_visible(),
            DiagramCell::TrustBoundaryCurve(a) => a.is_visible(),
            DiagramCell::TdTextBlock(a) => a.is_visible()
        }
    }
    fn get_shape (&self) -> String {
        match self {
            DiagramCell::Actor(a) => a.get_shape(),
            DiagramCell::Flow(a) => a.get_shape(),
            DiagramCell::Store(a) => a.get_shape(),
            DiagramCell::Process(a) => a.get_shape(),
            DiagramCell::TrustBoundaryBox(a) => a.get_shape(),
            DiagramCell::TrustBoundaryCurve(a) => a.get_shape(),
            DiagramCell::TdTextBlock(a) => a.get_shape()
        }
    }
    fn set_angle (&mut self, angle: f64) {
        match self {
            DiagramCell::Actor(a) => a.set_angle(angle),
            DiagramCell::Flow(a) => a.set_angle(angle),
            DiagramCell::Store(a) => a.set_angle(angle),
            DiagramCell::Process(a) => a.set_angle(angle),
            DiagramCell::TrustBoundaryBox(a) => a.set_angle(angle),
            DiagramCell::TrustBoundaryCurve(a) => a.set_angle(angle),
            DiagramCell::TdTextBlock(a) => a.set_angle(angle),
        };
    }
    fn set_visible (&mut self, visibility: bool) {
        match self {
            DiagramCell::Actor(a) => a.set_visible(visibility),
            DiagramCell::Flow(a) => a.set_visible(visibility),
            DiagramCell::Store(a) => a.set_visible(visibility),
            DiagramCell::Process(a) => a.set_visible(visibility),
            DiagramCell::TrustBoundaryBox(a) => a.set_visible(visibility),
            DiagramCell::TrustBoundaryCurve(a) => a.set_visible(visibility),
            DiagramCell::TdTextBlock(a) => a.set_visible(visibility),
        };
    }
}



impl Diagram {
    pub fn find_cell_id (&self, id: &str) -> Option<DiagramCell> {
        for cell in &self.cells {
            let cell_id = match cell {
                DiagramCell::Actor(el) => &el.id,
                DiagramCell::Flow(el) => &el.id,
                DiagramCell::Process(el) => &el.id,
                DiagramCell::Store(el) => &el.id,
                DiagramCell::TrustBoundaryBox(el) => &el.id,
                DiagramCell::TrustBoundaryCurve(el) => &el.id,
                DiagramCell::TdTextBlock(el) => &el.id
            };
            if cell_id == id {
                return Some(cell.clone());
            }
        }
        None
    }

    pub fn is_all_visible (&self) -> bool {
        for cell in &self.cells {
            if !cell.is_visible() {
                return false;
            }
        }
        true
    }
}
