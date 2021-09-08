pub mod line;
pub mod fetch;
pub mod address;
pub mod policies;

pub use self::fetch::CacheFetch;
pub use self::address::CacheAddress;

pub use self::policies::{
    LRUCache,
    FIFOCache,
    DirectCache,
};

pub use self::line::{
    LRUCacheLine,
    RegularCacheLine,
};

pub trait Cache {
    fn fetch (&mut self, address: CacheAddress) -> CacheFetch;
}