use crate::models::Project;
use crate::state::TableState;

#[derive(Debug)]
pub struct AppState {
    pub running: bool,
    pub projects_table: TableState,
    pub tasks_table: TableState,

    projects: Vec<Project>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            projects: Vec::new(),
            running: false,
            projects_table: Self::new_project_table(),
            tasks_table: TableState::new(),
        }
    }

    pub fn get_project(&self) -> &[Project] {
        &self.projects
    }

    pub fn set_projects(&mut self, projects: Vec<Project>) {
        self.projects_table.rows = self.project_rows(&projects);
        self.projects = projects;
    }

    fn new_project_table() -> TableState {
        let mut project_table = TableState::new();
        project_table.title = "Projects".to_string();

        project_table.header = vec![
            "Name".to_string(),
            "Tasks".to_string(),
            "Subprojects".to_string(),
        ];

        project_table
    }

    fn project_rows(&self, projects: &[Project]) -> Vec<Vec<String>> {
        projects
            .iter()
            .map(|project| {
                let tasks_count = project.tasks.len();
                let subprojects_count = project.subprojects.len();
                vec![
                    project.name.clone(),
                    tasks_count.to_string(),
                    subprojects_count.to_string(),
                ]
            })
            .collect()
    }
}
