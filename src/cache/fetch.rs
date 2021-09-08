use std::fmt::{Display, Formatter, Result};

#[derive(PartialEq, Debug)]
pub enum CacheFetch {
    Hit,
    Miss,
}

impl Display for CacheFetch {
    fn fmt (&self, f: &mut Formatter<'_>) -> Result {
        match self {
            CacheFetch::Hit => write!(f, "HIT"),
            CacheFetch::Miss => write!(f, "MISS"),
        }
    }
}