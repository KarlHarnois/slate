use crate::models::{ProgressStatus, Project, Task};
use crate::task_repo::{TaskRepository, TaskRepositoryError};
use regex::Regex;
use std::{
    fs,
    io::{BufRead, BufReader},
    path::{Path, PathBuf},
};
use walkdir::WalkDir;

#[derive(Debug)]
pub struct TaskFileRepository {
    root: PathBuf,
}

impl TaskFileRepository {
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self { root: root.into() }
    }

    fn parse_file(&self, path: &Path) -> Result<Project, TaskRepositoryError> {
        let file = fs::File::open(path)?;
        let reader = BufReader::new(file);

        let project_name = path
            .file_stem()
            .unwrap_or_default()
            .to_string_lossy()
            .into_owned();

        let mut project = Project {
            name: project_name,
            file_path: path.to_path_buf(),
            tasks: Vec::new(),
        };

        let regex_task = Regex::new(r"^\s*-\s*\[(.| )\]\s+(.*)$").unwrap();

        for line in reader.lines() {
            let line = line?;

            if let Some(capture) = regex_task.captures(&line) {
                let name = capture.get(2).unwrap().as_str().trim().to_string();
                let marker = capture.get(1).unwrap().as_str();

                let status = match marker {
                    "x" | "X" => ProgressStatus::Done,
                    "/" => ProgressStatus::Started,
                    _ => ProgressStatus::Pending,
                };

                let task = Task { name, status };
                project.tasks.push(task);
            }
        }

        Ok(project)
    }
}

impl TaskRepository for TaskFileRepository {
    fn fetch_projects(&self) -> Result<Vec<Project>, TaskRepositoryError> {
        let mut out = Vec::new();

        for entry in WalkDir::new(&self.root).into_iter().filter_map(Result::ok)
        {
            let path = entry.path();

            if path.is_file()
                && path.extension().map(|e| e == "md").unwrap_or(false)
            {
                out.push(self.parse_file(path)?);
            }
        }

        Ok(out)
    }
}
