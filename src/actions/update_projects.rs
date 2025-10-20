use crate::actions::Action;
use crate::models::Project;
use crate::states::{AppState, RowState};

pub struct UpdateProjects {
    pub projects: Vec<Project>,
}

impl Action for UpdateProjects {
    fn apply(self: Box<Self>, state: &mut AppState) {
        state.projects_table.rows = self.project_rows();

        let project_index = state.selected_project_index;

        let Some(project) = self.projects.get(project_index) else {
            return;
        };

        state.tasks_table.rows = self.task_rows(project);
        state.projects_table.is_focused = false;
        state.tasks_table.is_focused = true;
        state.projects = self.projects;
    }
}

impl UpdateProjects {
    fn project_rows(&self) -> Vec<RowState> {
        self.projects
            .iter()
            .enumerate()
            .map(|(index, project)| {
                let mut row = RowState::from(project);
                if index == 0 {
                    row.add_checkmark();
                }
                row
            })
            .collect()
    }

    fn task_rows(&self, project: &Project) -> Vec<RowState> {
        project.tasks.iter().map(RowState::from).collect()
    }
}
