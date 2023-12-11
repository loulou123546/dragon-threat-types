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
pub struct ThreatModelV2 {
    pub summary: Summary,
    pub detail: Detail,
    pub version: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Summary {
    pub title: Option<String>,
    pub owner: Option<String>,
    pub description: Option<String>,
    pub id: Option<usize>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[allow(non_snake_case)]
pub struct Detail {
    pub contributors: Option<Vec<Contributor>>,
    pub reviewer: Option<String>,
    pub diagramTop: Option<usize>,
    pub threatTop: Option<usize>,
    pub diagrams: Option<Vec<Diagram>>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Contributor {
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[allow(non_snake_case)]
pub struct Diagram {
    pub cells: Vec<DiagramCell>,
    pub version: Option<String>,
    pub title: Option<String>,
    pub descrition: Option<String>,
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

    pub fn filter_by_type (&self, kind: &DiagramCell) -> Vec<DiagramCell> {
        let mut filtered_cell = vec![];

        for cell in &self.cells {
            let keep = match cell {
                kind => true,
                _ => false,
            };
            if keep {
                filtered_cell.push(cell.clone());
            }
        }
        filtered_cell
    }
}
