use std::fmt::{
    Result,
    Display,
    Formatter,
};

#[derive(Debug)]
pub enum PolicyError {
    Unknown,
}

impl Display for PolicyError {
    fn fmt (&self, f: &mut Formatter<'_>) -> Result {
        match self {
            PolicyError::Unknown => write!(f, "Valor desconhecido para a política de substituição."),
        }
    }
}