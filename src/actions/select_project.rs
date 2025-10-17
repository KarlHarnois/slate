use crate::actions::Action;
use crate::states::AppState;

pub struct SelectProject;

impl Action for SelectProject {
    fn apply(self: Box<Self>, state: &mut AppState) {
        let Some(new_index) = state.projects_table.selected_row else {
            return;
        };
        state.selected_project_index = new_index;
        self.rebuild_task_table(state, new_index);
    }
}

impl SelectProject {
    fn rebuild_task_table(self, state: &mut AppState, project_index: usize) {
        let Some(project) = state.projects.get(project_index) else {
            return;
        };
        state.tasks_table.rows =
            project.tasks.iter().map(|task| task.to_row()).collect();

        state.projects_table.is_focused = false;
        state.tasks_table.is_focused = true;
        state.tasks_table.selected_row = Some(0);
    }
}
