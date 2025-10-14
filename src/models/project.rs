use crate::models::Task;
use std::path::PathBuf;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Project {
    pub name: String,
    pub file_path: PathBuf,
    pub tasks: Vec<Task>,
}

#[allow(dead_code)]
impl Project {
    pub fn new() -> Self {
        Self {
            name: String::new(),
            file_path: PathBuf::new(),
            tasks: vec![],
        }
    }
}
