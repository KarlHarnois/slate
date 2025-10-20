use crate::actions::Action;
use crate::models::{KeyBinding, KeyBindingContext, Project};
use crate::states::{ModalState, ModalType, TableState, TableType};

#[derive(Debug)]
pub struct AppState {
    pub is_running: bool,
    pub projects_table: TableState,
    pub projects: Vec<Project>,
    pub tasks_table: TableState,
    pub selected_project_index: usize,
    pub modal: Option<ModalState>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            projects: Vec::new(),
            is_running: false,
            projects_table: TableState::new(TableType::Projects),
            tasks_table: TableState::new(TableType::Tasks),
            selected_project_index: 0,
            modal: None,
        }
    }

    pub fn apply<A: Action>(&mut self, action: A) {
        Box::new(action).apply(self);
    }

    pub fn focused_table_mut(&mut self) -> &mut TableState {
        if self.projects_table.is_focused {
            &mut self.projects_table
        } else {
            &mut self.tasks_table
        }
    }

    pub fn keybindings(&self) -> Vec<String> {
        let Some(context) = self.keybinding_context() else {
            return Vec::new();
        };
        KeyBinding::list_for(context)
            .iter()
            .map(|binding| binding.to_string())
            .collect()
    }

    fn keybinding_context(&self) -> Option<KeyBindingContext> {
        if let Some(modal) = self.modal.as_ref() {
            match modal.modal_type {
                ModalType::NewProject => {
                    return Some(KeyBindingContext::NewProject);
                }
                ModalType::NewTask => return Some(KeyBindingContext::NewTask),
            }
        }

        if self.projects_table.is_focused {
            Some(KeyBindingContext::Projects)
        } else if self.tasks_table.is_focused {
            Some(KeyBindingContext::Tasks)
        } else {
            None
        }
    }
}
