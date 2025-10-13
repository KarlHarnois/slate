use crate::models::{Project, Subproject, Task, TaskStatus};
use regex::Regex;
use std::{
    error::Error,
    fmt,
    fmt::{Debug, Display, Formatter},
    fs,
    io::{BufRead, BufReader},
    path::{Path, PathBuf},
};
use walkdir::WalkDir;

#[derive(Debug)]
pub enum TaskRepositoryError {
    IO(std::io::Error),
    Other(String),
}

impl Display for TaskRepositoryError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        match self {
            TaskRepositoryError::IO(error) => write!(formatter, "IO error: {error}"),
            TaskRepositoryError::Other(msg) => write!(formatter, "Other: {msg}"),
        }
    }
}

impl Error for TaskRepositoryError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            TaskRepositoryError::IO(error) => Some(error),
            TaskRepositoryError::Other(_) => None,
        }
    }
}

impl From<std::io::Error> for TaskRepositoryError {
    fn from(error: std::io::Error) -> Self {
        TaskRepositoryError::IO(error)
    }
}

pub trait TaskRepository: Debug {
    fn fetch_projects(&self) -> Result<Vec<Project>, TaskRepositoryError>;
    // fn save_project(&self, project: Project) -> Result<Project, TaskRepositoryError>;
}

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
            subprojects: Vec::new(),
            tasks: Vec::new(),
        };

        let regex_heading = Regex::new(r"^(#{1,6})\s+(.*)$").unwrap();
        let regex_task = Regex::new(r"^\s*-\s*\[(.| )\]\s+(.*)$").unwrap();

        for line in reader.lines() {
            let line = line?;

            if let Some(capture) = regex_heading.captures(&line) {
                let name = capture.get(2).unwrap().as_str().trim().to_string();

                project.subprojects.push(Subproject {
                    name: name,
                    subprojects: Vec::new(),
                    tasks: Vec::new(),
                });

                continue;
            }

            if let Some(capture) = regex_task.captures(&line) {
                let name = capture.get(2).unwrap().as_str().trim().to_string();
                let marker = capture.get(1).unwrap().as_str();

                let status = match marker {
                    "x" | "X" => TaskStatus::Done,
                    "/" => TaskStatus::Started,
                    _ => TaskStatus::Pending,
                };

                project.subprojects.last_mut().unwrap().tasks.push(Task {
                    name: name,
                    status: status,
                });
            }
        }

        Ok(project)
    }
}

impl TaskRepository for TaskFileRepository {
    fn fetch_projects(&self) -> Result<Vec<Project>, TaskRepositoryError> {
        let mut out = Vec::new();

        for entry in WalkDir::new(&self.root).into_iter().filter_map(Result::ok) {
            let path = entry.path();

            if path.is_file() && path.extension().map(|e| e == "md").unwrap_or(false) {
                out.push(self.parse_file(path)?);
            }
        }

        Ok(out)
    }
}
