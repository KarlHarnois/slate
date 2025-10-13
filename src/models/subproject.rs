use crate::models::Task;

#[derive(Debug, Clone)]
pub struct Subproject {
    pub name: String,
    pub children: Vec<Subproject>,
    pub tasks: Vec<Task>,
}
