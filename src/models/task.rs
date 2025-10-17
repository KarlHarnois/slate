use crate::models::ProgressStatus;
use crate::states::{RowEmphasis, RowState};

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
            emphasis: match self.status {
                ProgressStatus::Done => RowEmphasis::Low,
                ProgressStatus::Pending => RowEmphasis::Medium,
                ProgressStatus::Started => RowEmphasis::High,
            },
        }
    }
}
