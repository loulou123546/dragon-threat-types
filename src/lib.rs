pub struct ThreatModelV2 {
    pub summary: Summary
}

pub struct Summary {
    pub title: Option<String>,
    pub owner: Option<String>,
    pub description: Option<String>,
    pub id: Option<usize>
}
