use crate::states::TextInput;

#[derive(Debug)]
pub enum ModalType {
    NewTask,
    NewProject,
}

#[derive(Debug)]
pub struct ModalState {
    pub modal_type: ModalType,
    pub text: TextInput,
}

impl ModalState {
    pub fn title(&self) -> String {
        match self.modal_type {
            ModalType::NewTask => "New Task".to_string(),
            ModalType::NewProject => "New Project".to_string(),
        }
    }
}
