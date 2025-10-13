use crate::actions::Action;
use crate::models::Project;
use crate::states::{TableState, TableType};

#[derive(Debug)]
pub struct AppState {
    pub is_running: bool,
    pub projects_table: TableState,
    pub tasks_table: TableState,

    projects: Vec<Project>,
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
        action.apply(self);
    }

    pub fn set_projects(&mut self, projects: Vec<Project>) {
        self.projects_table.rows = self.project_rows(&projects);

        if let Some(project) = projects.first() {
            self.tasks_table.rows = self.task_rows(&project);
        }

        self.projects_table.is_focused = true;
        self.projects = projects;
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

    fn task_rows(&self, project: &Project) -> Vec<Vec<String>> {
        project
            .tasks
            .iter()
            .map(|task| vec![task.status.label(), task.name.clone()])
            .collect()
    }
}
