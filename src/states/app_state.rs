use crate::actions::Action;
use crate::models::Project;
use crate::states::{ModalState, TableState, TableType};

#[derive(Debug)]
pub struct AppState {
    pub is_running: bool,
    pub projects_table: TableState,
    pub projects: Vec<Project>,
    pub tasks_table: TableState,
    pub selected_project_index: usize,
    pub modal: Option<ModalState>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            projects: Vec::new(),
            is_running: false,
            projects_table: Self::new_project_table(),
            tasks_table: Self::new_tasks_table(),
            selected_project_index: 0,
            modal: None,
        }
    }

    pub fn apply<A: Action>(&mut self, action: A) {
        Box::new(action).apply(self);
    }

    pub fn focused_table_mut(&mut self) -> &mut TableState {
        if self.projects_table.is_focused {
            &mut self.projects_table
        } else {
            &mut self.tasks_table
        }
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
