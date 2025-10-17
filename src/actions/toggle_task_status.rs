use crate::actions::Action;
use crate::states::{AppState, RowState};

pub struct ToggleTaskStatus;

impl Action for ToggleTaskStatus {
    fn apply(self: Box<Self>, state: &mut AppState) {
        let Some(task_index) = state.tasks_table.selected_row else {
            return;
        };
        let Some(new_row) = self.toggle_task_and_create_row(state, task_index)
        else {
            return;
        };
        state.tasks_table.rows[task_index] = new_row;
    }
}

impl ToggleTaskStatus {
    fn toggle_task_and_create_row(
        self,
        state: &mut AppState,
        task_index: usize,
    ) -> Option<RowState> {
        let project_index = state.selected_project_index;
        let project = state.projects.get_mut(project_index)?;
        let task = project.tasks.get_mut(task_index)?;
        task.status = task.status.next();
        Some(task.to_row())
    }
}

#[cfg(test)]
mod tests {
    use crate::actions::{Action, ToggleTaskStatus, UpdateProjects};
    use crate::models::{ProgressStatus, Project, Task};
    use crate::states::{AppState, RowEmphasis, RowState};

    #[test]
    fn updates_the_correct_task() {
        let mut state = AppState::new();
        let mut project = Project::new();

        state.projects_table.is_focused = false;
        state.tasks_table.is_focused = true;
        state.tasks_table.selected_row = Some(1);

        project.tasks = vec![Task::new(), Task::new(), Task::new()];

        if let Some(task) = project.tasks.get_mut(1) {
            task.name = "Buy Food".to_string();
            task.status = ProgressStatus::Started;
        }

        Box::new(UpdateProjects {
            projects: vec![project],
        })
        .apply(&mut state);

        Box::new(ToggleTaskStatus).apply(&mut state);

        assert_eq!(
            state.projects[0].tasks[1].status,
            ProgressStatus::Done,
            "Task status should be updated"
        );

        assert_eq!(
            state.tasks_table.rows[1],
            RowState {
                cells: vec!["Done".to_string(), "Buy Food".to_string()],
                emphasis: RowEmphasis::Low
            },
            "Task cells should be updated"
        );
    }
}
