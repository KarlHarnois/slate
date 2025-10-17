use crate::models::ProgressStatus;
use crate::states::RowState;

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

    pub fn to_row(&self) -> RowState {
        RowState {
            cells: vec![self.status.label(), self.name.to_string()],
            is_crossed_out: self.status == ProgressStatus::Done,
            is_emphasized: self.status == ProgressStatus::Started,
        }
    }
}
