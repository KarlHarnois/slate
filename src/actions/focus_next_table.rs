use crate::actions::Action;
use crate::states::AppState;

pub struct FocusNextTable {}

impl Action for FocusNextTable {
    fn apply(&self, state: &mut AppState) {
        let tasks_should_focus = state.projects_table.is_focused;

        state.projects_table.is_focused = !tasks_should_focus;
        state.tasks_table.is_focused = tasks_should_focus;
    }
}
