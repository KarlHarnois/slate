use crate::actions::Action;
use crate::models::Project;
use crate::states::{TableState, TableType};

#[derive(Debug)]
pub struct AppState {
    pub is_running: bool,
    pub projects_table: TableState,
    pub projects: Vec<Project>,
    pub tasks_table: TableState,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            projects: Vec::new(),
            is_running: false,
            projects_table: Self::new_project_table(),
            tasks_table: Self::new_tasks_table(),
        }
    }

    pub fn apply<A: Action>(&mut self, action: A) {
        Box::new(action).apply(self);
    }

    fn new_project_table() -> TableState {
        let mut table = TableState::new(TableType::Projects);

        table.header = vec![
            "Name".to_string(),
            "Tasks".to_string(),
            "Subprojects".to_string(),
        ];

        table
    }

    fn new_tasks_table() -> TableState {
        let mut table = TableState::new(TableType::Tasks);

        table.header = vec!["Status".to_string(), "Name".to_string()];

        table
    }
}
