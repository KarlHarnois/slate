#[derive(Debug, PartialEq)]
pub enum RowEmphasis {
    Low,
    Medium,
    High,
}

#[derive(Debug, PartialEq)]
pub struct RowState {
    pub cells: Vec<String>,
    pub emphasis: RowEmphasis,
}

impl RowState {
    pub fn new(cells: Vec<String>) -> Self {
        Self {
            cells,
            emphasis: RowEmphasis::Medium,
        }
    }
}
