use crate::actions::{
    Action, ActionFactory, DeleteChar, FocusNextTable, InsertChar,
    MoveDownInTable, MoveLeftInTextField, MoveRightInTextField, MoveUpInTable,
    NoOp, ToggleTaskStatus,
    modals::{CancelModal, ShowNewProjectModal, ShowNewTaskModal},
    quit_app::QuitApp,
    select_project::SelectProject,
};
use crate::states::AppState;

use crossterm::event::{
    KeyCode::{BackTab, Backspace, Char, Down, Esc, Left, Right, Tab, Up},
    KeyEvent, KeyModifiers,
};

pub struct HandleKeyEvent {
    pub key: KeyEvent,
}

impl ActionFactory for HandleKeyEvent {
    fn create(&self, state: &AppState) -> Box<dyn Action> {
        if let Some(action) = self.global_actions() {
            return action;
        }

        if state.modal.is_some() {
            self.modal_actions()
        } else {
            self.home_actions(state)
        }
    }
}

impl HandleKeyEvent {
    fn global_actions(&self) -> Option<Box<dyn Action>> {
        match (self.key.modifiers, self.key.code) {
            (KeyModifiers::CONTROL, Char('c') | Char('C')) => {
                Some(Box::new(QuitApp))
            }
            _ => None,
        }
    }

    fn modal_actions(&self) -> Box<dyn Action> {
        match (self.key.modifiers, self.key.code) {
            (_, Esc) => Box::new(CancelModal),
            (_, Char(char)) => Box::new(InsertChar { char }),
            (_, Backspace) => Box::new(DeleteChar),
            (_, Left) => Box::new(MoveLeftInTextField),
            (_, Right) => Box::new(MoveRightInTextField),
            _ => Box::new(NoOp),
        }
    }

    fn home_actions(&self, state: &AppState) -> Box<dyn Action> {
        match (self.key.modifiers, self.key.code) {
            (_, Esc | Char('q')) => Box::new(QuitApp),
            (KeyModifiers::NONE, Tab) => Box::new(FocusNextTable),
            (KeyModifiers::SHIFT, Tab) => Box::new(FocusNextTable),
            (_, BackTab) => Box::new(FocusNextTable),
            (_, Char('a')) => {
                if state.tasks_table.is_focused {
                    Box::new(ShowNewTaskModal)
                } else {
                    Box::new(ShowNewProjectModal)
                }
            }
            (_, Char('k')) => Box::new(MoveUpInTable),
            (_, Char('j')) => Box::new(MoveDownInTable),
            (_, Up) => Box::new(MoveUpInTable),
            (_, Down) => Box::new(MoveDownInTable),
            (_, Char(' ')) => {
                if state.tasks_table.is_focused {
                    Box::new(ToggleTaskStatus)
                } else {
                    Box::new(SelectProject)
                }
            }
            _ => Box::new(NoOp),
        }
    }
}
