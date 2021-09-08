use crate::source::error::SourceError;

#[derive(Debug)]
pub enum ApplicationError {
    Source (SourceError),
    IO (std::io::Error),
}

impl From<std::io::Error> for ApplicationError {
    fn from (error: std::io::Error) -> ApplicationError {
        ApplicationError::IO(error)
    }
}

impl From<SourceError> for ApplicationError {
    fn from (error: SourceError) -> ApplicationError {
        ApplicationError::Source(error)
    }
}