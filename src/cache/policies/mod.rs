mod lru;
mod fifo;
mod direct;

pub use self::lru::LRUCache;
pub use self::fifo::FIFOCache;
pub use self::direct::DirectCache;