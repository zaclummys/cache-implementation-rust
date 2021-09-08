mod error;

use std::str::FromStr;

use crate::cache::{
    Cache,
    LRUCache,
    FIFOCache,
    DirectCache,
};

pub use self::error::PolicyError;

pub enum Policy {
    DIR,
    LRU,
    FIFO,
}

impl Policy {
    pub fn build (&self, cache_total_lines: usize, cache_line_size: usize) -> Box<dyn Cache> {
        match self {
            Policy::LRU => Box::new(LRUCache::new(cache_total_lines, cache_line_size)),
            Policy::FIFO => Box::new(FIFOCache::new(cache_total_lines, cache_line_size)),
            Policy::DIR => Box::new(DirectCache::new(cache_total_lines, cache_line_size)),
        }
    }
}

impl FromStr for Policy {
    type Err = PolicyError;

    fn from_str (string: &str) -> Result<Policy, PolicyError> {
        match string.to_ascii_lowercase().as_str() {
            "dir" => Ok(Policy::DIR),
            "lru" => Ok(Policy::LRU),
            "fifo" => Ok(Policy::FIFO),
            _ => Err(PolicyError::Unknown),
        }
    }
}