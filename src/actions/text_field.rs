use crate::actions::Action;
use crate::states::AppState;

pub struct InsertChar {
    pub char: char,
}

pub struct DeleteChar;
pub struct MoveLeftInTextField;
pub struct MoveRightInTextField;

impl Action for InsertChar {
    fn apply(self: Box<Self>, state: &mut AppState) {
        if let Some(modal) = state.modal.as_mut() {
            modal.text.insert(self.char);
        };
    }
}

impl Action for DeleteChar {
    fn apply(self: Box<Self>, state: &mut AppState) {
        if let Some(modal) = state.modal.as_mut() {
            modal.text.delete_char();
        };
    }
}

impl Action for MoveLeftInTextField {
    fn apply(self: Box<Self>, state: &mut AppState) {
        if let Some(modal) = state.modal.as_mut() {
            modal.text.move_cursor_left();
        };
    }
}

impl Action for MoveRightInTextField {
    fn apply(self: Box<Self>, state: &mut AppState) {
        if let Some(modal) = state.modal.as_mut() {
            modal.text.move_cursor_right()
        };
    }
}
