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
}
