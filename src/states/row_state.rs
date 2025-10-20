use crate::models::{ProgressStatus, Project, Task};

#[derive(Debug, PartialEq)]
pub enum RowEmphasis {
    Low,
    Medium,
    High,
}

#[derive(Debug, PartialEq)]
pub struct RowState {
    pub cells: Vec<String>,
    pub emphasis: RowEmphasis,
}

impl RowState {
    pub fn add_checkmark(&mut self) {
        self.cells[0] = "✔".to_string();
    }

    pub fn remove_checkmark(&mut self) {
        self.cells[0] = String::new();
    }
}

impl From<&Task> for RowState {
    fn from(task: &Task) -> Self {
        Self {
            cells: vec![task.status.label(), task.name.to_string()],
            emphasis: match task.status {
                ProgressStatus::Done => RowEmphasis::Low,
                ProgressStatus::Pending => RowEmphasis::Medium,
                ProgressStatus::Started => RowEmphasis::High,
            },
        }
    }
}

impl From<&Project> for RowState {
    fn from(project: &Project) -> Self {
        Self {
            cells: vec![String::new(), project.name.to_string()],
            emphasis: RowEmphasis::Medium,
        }
    }
}
