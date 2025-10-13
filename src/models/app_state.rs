use crate::models::Project;

#[derive(Debug)]
pub enum FocusedBlock {
    Projects,
    Tasks,
}

#[derive(Debug)]
pub struct AppState {
    pub focused_block: FocusedBlock,
    pub projects: Vec<Project>,
    pub running: bool,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            focused_block: FocusedBlock::Projects,
            projects: Vec::new(),
            running: false,
        }
    }
}
