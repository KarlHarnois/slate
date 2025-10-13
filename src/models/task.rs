#[derive(Debug, Clone)]
pub enum TaskStatus {
    Pending,
    Started,
    Done,
}

#[derive(Debug, Clone)]
pub struct Task {
    pub name: String,
    pub status: TaskStatus,
}
