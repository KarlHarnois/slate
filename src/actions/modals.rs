use crate::actions::Action;
use crate::states::{AppState, ModalState};

pub struct ShowNewTaskModal;
pub struct ShowNewProjectModal;

impl Action for ShowNewTaskModal {
    fn apply(self: Box<Self>, state: &mut AppState) {
        state.modal = Some(ModalState::new());
    }
}

impl Action for ShowNewProjectModal {
    fn apply(self: Box<Self>, state: &mut AppState) {
        state.modal = Some(ModalState::new());
    }
}
