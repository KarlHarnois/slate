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
        let Some(modal) = state.modal.as_mut() else {
            return;
        };

        modal.text.insert(self.char);
    }
}

impl Action for DeleteChar {
    fn apply(self: Box<Self>, state: &mut AppState) {
        let Some(modal) = state.modal.as_mut() else {
            return;
        };

        if modal.text.cursor == 0 {
            return;
        }
        let before_char_to_delete =
            modal.text.value.chars().take(modal.text.cursor - 1);

        let after_char_to_delete =
            modal.text.value.chars().skip(modal.text.cursor);

        modal.text.value =
            before_char_to_delete.chain(after_char_to_delete).collect();

        modal.text.cursor -= 1;
    }
}

impl Action for MoveLeftInTextField {
    fn apply(self: Box<Self>, state: &mut AppState) {
        let Some(modal) = state.modal.as_mut() else {
            return;
        };

        if modal.text.cursor == 0 {
            return;
        };

        modal.text.cursor -= 1;
    }
}

impl Action for MoveRightInTextField {
    fn apply(self: Box<Self>, state: &mut AppState) {
        let Some(modal) = state.modal.as_mut() else {
            return;
        };

        if modal.text.cursor == modal.text.value.len() {
            return;
        };

        modal.text.cursor += 1;
    }
}
