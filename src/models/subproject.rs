use crate::models::Task;

#[derive(Debug, Clone)]
pub struct Subproject {
    pub name: String,
    pub subprojects: Vec<Subproject>,
    pub tasks: Vec<Task>,
}
