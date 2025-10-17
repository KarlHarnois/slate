#[derive(Debug)]
pub struct RowState {
    pub cells: Vec<String>,
    pub is_crossed_out: bool,
    pub is_emphasized: bool,
}

impl RowState {
    pub fn new(cells: Vec<String>) -> Self {
        Self {
            cells,
            is_crossed_out: false,
            is_emphasized: false,
        }
    }
}
