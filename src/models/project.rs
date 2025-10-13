use crate::models::{Subproject, Task};
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Project {
    pub name: String,
    pub file_path: PathBuf,
    pub subprojects: Vec<Subproject>,
    pub tasks: Vec<Task>,
}
