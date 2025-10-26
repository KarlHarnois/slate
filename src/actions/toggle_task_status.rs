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
        Some(RowState::from(&*task))
    }
}

#[cfg(test)]
mod tests {
    use crate::actions::{Action, ToggleTaskStatus, UpdateProjects};
    use crate::models::{ProgressStatus, Project, Task};
    use crate::states::{AppState, RowEmphasis, RowState};

    fn tasks() -> Vec<Task> {
        vec![
            Task {
                name: "Read Emails".to_string(),
                status: ProgressStatus::Pending,
            },
            Task {
                name: "Buy Food".to_string(),
                status: ProgressStatus::Started,
            },
            Task {
                name: "Clean Kitchen".to_string(),
                status: ProgressStatus::Done,
            },
        ]
    }

    fn project() -> Project {
        let mut project = Project::new();
        project.tasks = tasks();
        project
    }

    fn app_state() -> AppState {
        let mut state = AppState::new();
        let projects = vec![project()];

        state.projects_table.is_focused = false;
        state.tasks_table.is_focused = true;

        Box::new(UpdateProjects { projects }).apply(&mut state);

        state
    }

    fn task_statuses(state: &AppState) -> Vec<ProgressStatus> {
        state.projects[0]
            .tasks
            .iter()
            .map(|task| task.status)
            .collect()
    }

    #[test]
    fn updates_pending_tasks_to_started() {
        let mut state = app_state();

        state.tasks_table.selected_row = Some(0);
        Box::new(ToggleTaskStatus).apply(&mut state);

        assert_eq!(
            task_statuses(&state),
            vec![
                ProgressStatus::Started,
                ProgressStatus::Started,
                ProgressStatus::Done,
            ],
            "Task should be started"
        );
    }

    #[test]
    fn updates_started_tasks_to_done() {
        let mut state = app_state();

        state.tasks_table.selected_row = Some(1);
        Box::new(ToggleTaskStatus).apply(&mut state);

        assert_eq!(
            task_statuses(&state),
            vec![
                ProgressStatus::Pending,
                ProgressStatus::Done,
                ProgressStatus::Done,
            ],
            "Task should be done"
        );
    }

    #[test]
    fn updates_done_tasks_to_pending() {
        let mut state = app_state();

        state.tasks_table.selected_row = Some(2);
        Box::new(ToggleTaskStatus).apply(&mut state);

        assert_eq!(
            task_statuses(&state),
            vec![
                ProgressStatus::Pending,
                ProgressStatus::Started,
                ProgressStatus::Pending
            ],
            "Task should be pending"
        );
    }

    #[test]
    fn updates_the_row_state() {
        let mut state = app_state();

        state.tasks_table.selected_row = Some(1);
        Box::new(ToggleTaskStatus).apply(&mut state);

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
