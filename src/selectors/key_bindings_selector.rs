use crate::models::{KeyBinding, KeyBindingContext};
use crate::selectors::Selector;
use crate::states::{AppState, ModalType};

pub struct KeyBindingsSelector;

impl Selector<Vec<String>> for KeyBindingsSelector {
    fn select(&self, state: &AppState) -> Vec<String> {
        let Some(context) = self.keybinding_context(state) else {
            return Vec::new();
        };
        KeyBinding::list_for(context)
            .iter()
            .map(ToString::to_string)
            .collect()
    }
}

impl KeyBindingsSelector {
    fn keybinding_context(
        &self,
        state: &AppState,
    ) -> Option<KeyBindingContext> {
        if let Some(modal) = state.modal.as_ref() {
            match modal.modal_type {
                ModalType::NewProject => {
                    return Some(KeyBindingContext::NewProject);
                }
                ModalType::NewTask => return Some(KeyBindingContext::NewTask),
            }
        }

        if state.projects_table.is_focused {
            Some(KeyBindingContext::Projects)
        } else if state.tasks_table.is_focused {
            Some(KeyBindingContext::Tasks)
        } else {
            None
        }
    }
}
