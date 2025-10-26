use crate::actions::Action;
use crate::states::{AppState, ModalState, ModalType, TextInput};

pub struct ShowNewTaskModal;
pub struct ShowNewProjectModal;
pub struct CancelModal;

impl Action for ShowNewTaskModal {
    fn apply(self: Box<Self>, state: &mut AppState) {
        state.projects_table.is_focused = false;
        state.tasks_table.is_focused = false;

        state.modal = Some(ModalState {
            modal_type: ModalType::NewTask,
            text: TextInput::new(),
        });
    }
}

impl Action for ShowNewProjectModal {
    fn apply(self: Box<Self>, state: &mut AppState) {
        state.projects_table.is_focused = false;
        state.tasks_table.is_focused = false;

        state.modal = Some(ModalState {
            modal_type: ModalType::NewProject,
            text: TextInput::new(),
        });
    }
}

impl Action for CancelModal {
    fn apply(self: Box<Self>, state: &mut AppState) {
        let Some(modal) = state.modal.take() else {
            return;
        };

        match modal.modal_type {
            ModalType::NewProject => state.projects_table.is_focused = true,
            ModalType::NewTask => state.tasks_table.is_focused = true,
        };
    }
}
