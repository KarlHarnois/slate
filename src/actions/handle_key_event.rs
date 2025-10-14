use crate::actions::{
    Action, ActionFactory, MoveDownInTable, MoveUpInTable, ToggleTaskStatus,
    focus_next_table::FocusNextTable, noop::NoOp, quit_app::QuitApp,
};
use crate::states::AppState;

use crossterm::event::{
    KeyCode::{BackTab, Char, Down, Esc, Tab, Up},
    KeyEvent, KeyModifiers,
};

pub struct HandleKeyEvent {
    pub key: KeyEvent,
}

impl ActionFactory for HandleKeyEvent {
    fn create(&self, state: &AppState) -> Box<dyn Action> {
        match (self.key.modifiers, self.key.code) {
            (_, Esc | Char('q')) => Box::new(QuitApp),
            (KeyModifiers::CONTROL, Char('c') | Char('C')) => Box::new(QuitApp),
            (KeyModifiers::NONE, Tab) => Box::new(FocusNextTable),
            (KeyModifiers::SHIFT, Tab) => Box::new(FocusNextTable),
            (_, BackTab) => Box::new(FocusNextTable),
            (_, Char('k')) => Box::new(MoveUpInTable),
            (_, Char('j')) => Box::new(MoveDownInTable),
            (_, Up) => Box::new(MoveUpInTable),
            (_, Down) => Box::new(MoveDownInTable),
            (_, Char(' ')) => {
                if state.tasks_table.is_focused {
                    Box::new(ToggleTaskStatus)
                } else {
                    Box::new(FocusNextTable)
                }
            }
            _ => Box::new(NoOp),
        }
    }
}
