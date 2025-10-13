use std::path::PathBuf;
use crate::models::{Task, Subproject};

#[derive(Debug, Clone)]
pub struct Project {
    pub name: String,
    pub file_path: PathBuf,
    pub children: Vec<Subproject>,
    pub tasks: Vec<Task>,
}
