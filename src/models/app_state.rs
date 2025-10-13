use crate::models::Project;

#[derive(Debug)]
pub struct AppState {
    pub running: bool,
    pub projects: Vec<Project>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            running: false,
            projects: Vec::new(),
        }
    }
}
