use crate::actions::{
    Action, ActionFactory, focus_next_table::FocusNextTable, noop::NoOp,
    quit_app::QuitApp,
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
            (_, Esc | Char('q')) => Box::new(QuitApp),
            (KeyModifiers::CONTROL, Char('c') | Char('C')) => Box::new(QuitApp),
            (KeyModifiers::NONE, Tab) => Box::new(FocusNextTable),
            (KeyModifiers::SHIFT, Tab) => Box::new(FocusNextTable),
            (_, BackTab) => Box::new(FocusNextTable),
            _ => Box::new(NoOp),
        }
    }
}
