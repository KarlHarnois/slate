use std::{
    error::Error,
    fmt,
    fmt::{Display, Formatter},
};

#[derive(Debug)]
pub enum TaskRepositoryError {
    IO(std::io::Error),
}

impl Display for TaskRepositoryError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        match self {
            TaskRepositoryError::IO(error) => {
                write!(formatter, "IO error: {error}")
            }
        }
    }
}

impl Error for TaskRepositoryError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            TaskRepositoryError::IO(error) => Some(error),
        }
    }
}

impl From<std::io::Error> for TaskRepositoryError {
    fn from(error: std::io::Error) -> Self {
        TaskRepositoryError::IO(error)
    }
}
