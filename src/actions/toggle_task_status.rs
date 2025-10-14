use crate::actions::{Action, ActionFactory, NoOp, UpdateProjects};
use crate::states::AppState;

pub struct ToggleTaskStatus;

impl ActionFactory for ToggleTaskStatus {
    fn create(&self, state: &AppState) -> Box<dyn Action> {
        let table = state.focused_table();
        let mut projects = state.projects.clone();

        if let Some(index) = table.selected_row
            && let Some(project) = projects.get_mut(0)
            && let Some(task) = project.tasks.get_mut(index)
        {
            task.status = task.status.clone().next();

            return Box::new(UpdateProjects { projects });
        }

        Box::new(NoOp)
    }
}
