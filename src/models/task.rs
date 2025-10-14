use crate::models::ProgressStatus;

#[derive(Debug, Clone)]
pub struct Task {
    pub name: String,
    pub status: ProgressStatus,
}

#[allow(dead_code)]
impl Task {
    pub fn new() -> Self {
        Self {
            name: String::new(),
            status: ProgressStatus::Pending,
        }
    }

    pub fn to_row(&self) -> Vec<String> {
        vec![self.status.label(), self.name.to_string()]
    }
}
