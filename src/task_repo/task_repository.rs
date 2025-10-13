use crate::models::Project;
use crate::task_repo::TaskRepositoryError;
use std::fmt::Debug;

pub trait TaskRepository: Debug {
    fn fetch_projects(&self) -> Result<Vec<Project>, TaskRepositoryError>;
}
