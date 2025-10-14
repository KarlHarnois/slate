#[derive(Debug, Clone)]
pub enum ProgressStatus {
    Pending,
    Started,
    Done,
}

impl ProgressStatus {
    pub fn label(&self) -> String {
        match self {
            Self::Pending => "Pending".to_string(),
            Self::Started => "Started".to_string(),
            Self::Done => "Done".to_string(),
        }
    }

    pub fn next(self) -> Self {
        match self {
            Self::Pending => Self::Started,
            Self::Started => Self::Done,
            Self::Done => Self::Pending,
        }
    }
}
