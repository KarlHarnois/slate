use crate::models::ProgressStatus;

#[derive(Debug, Clone)]
pub struct Task {
    pub name: String,
    pub status: ProgressStatus,
}
