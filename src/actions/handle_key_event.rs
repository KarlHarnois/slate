use crate::actions::{
    Action, ActionFactory, focus_next_table::FocusNextTable, noop::NoOp, quit::Quit,
};

use crossterm::event::{
    KeyCode::{BackTab, Char, Esc, Tab},
    KeyEvent, KeyModifiers,
};

pub struct HandleKeyEvent {
    pub key: KeyEvent,
}

impl ActionFactory for HandleKeyEvent {
    fn create(&self) -> Box<dyn Action> {
        match (self.key.modifiers, self.key.code) {
            (_, Esc | Char('q')) => Box::new(Quit),
            (KeyModifiers::CONTROL, Char('c') | Char('C')) => Box::new(Quit),
            (KeyModifiers::NONE, Tab) => Box::new(FocusNextTable),
            (KeyModifiers::SHIFT, Tab) => Box::new(FocusNextTable),
            (_, BackTab) => Box::new(FocusNextTable),
            _ => Box::new(NoOp),
        }
    }
}
