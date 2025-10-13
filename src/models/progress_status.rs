#[derive(Debug, Clone)]
pub enum ProgressStatus {
    Pending,
    Started,
    Done,
}

impl ProgressStatus {
    pub fn label(&self) -> String {
        match self {
            ProgressStatus::Pending => "Pending".to_string(),
            ProgressStatus::Started => "Started".to_string(),
            ProgressStatus::Done => "Done".to_string(),
        }
    }
}
