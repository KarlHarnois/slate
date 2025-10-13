use crate::actions::{Action, ActionFactory, FocusNextTable, NoOp, Quit};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

pub struct HandleKeyEvent {
    pub key: KeyEvent,
}

impl ActionFactory for HandleKeyEvent {
    fn create(&self) -> Box<dyn Action> {
        match (self.key.modifiers, self.key.code) {
            (_, KeyCode::Esc | KeyCode::Char('q'))
            | (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => Box::new(Quit {}),
            (KeyModifiers::NONE, KeyCode::Tab)
            | (KeyModifiers::SHIFT, KeyCode::Tab)
            | (_, KeyCode::BackTab) => Box::new(FocusNextTable {}),
            _ => Box::new(NoOp {}),
        }
    }
}
