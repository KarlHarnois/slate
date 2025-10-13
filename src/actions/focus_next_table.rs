use crate::actions::Action;
use crate::states::AppState;

pub struct FocusNextTable;

impl Action for FocusNextTable {
    fn apply(self: Box<Self>, state: &mut AppState) {
        let tasks_should_focus = state.projects_table.is_focused;

        state.projects_table.is_focused = !tasks_should_focus;
        state.tasks_table.is_focused = tasks_should_focus;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::states::AppState;

    #[test]
    fn toggles_from_projects_to_tasks() {
        let mut state = state_with_projects_focused(true);

        state.apply(FocusNextTable);

        assert!(
            !state.projects_table.is_focused,
            "projects should lose focus"
        );
        assert!(state.tasks_table.is_focused, "tasks should gain focus");
    }

    #[test]
    fn toggles_from_tasks_to_projects() {
        let mut state = state_with_projects_focused(false);

        state.apply(FocusNextTable);

        assert!(
            state.projects_table.is_focused,
            "projects should gain focus"
        );
        assert!(!state.tasks_table.is_focused, "tasks should lose focus");
    }

    fn state_with_projects_focused(is_focused: bool) -> AppState {
        let mut state = AppState::new();
        state.projects_table.is_focused = is_focused;
        state.tasks_table.is_focused = !is_focused;
        state
    }
}
