use std::collections::VecDeque;

use crate::cache::{
    Cache,
    CacheFetch,
    CacheAddress,
    RegularCacheLine,
};

pub struct FIFOCache {
    line_size: usize,
    lines: VecDeque<RegularCacheLine>,
}

impl FIFOCache {
    pub fn new (total_lines: usize, line_size: usize) -> FIFOCache {
        let mut lines = VecDeque::with_capacity(total_lines);

        for _ in 0..total_lines {
            lines.push_back(RegularCacheLine::invalid());
        }

        FIFOCache {
            lines,
            line_size,
        }
    }

    fn exists (&self, tag: usize) -> bool {
        self.lines.iter().any(|line| line.matches(tag))
    }

    fn insert (&mut self, tag: usize) {
        self.lines.pop_front();

        self.lines.push_back(RegularCacheLine::valid(tag));
    }
}

impl Cache for FIFOCache {
    fn fetch (&mut self, address: CacheAddress) -> CacheFetch {
        let tag = address.get_associative_location(self.line_size);

        if self.exists(tag) {
            CacheFetch::Hit
        } else {
            self.insert(tag);

            CacheFetch::Miss
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Cache, CacheFetch, FIFOCache};

    #[test]
    fn test_can_hit_after_miss () {
        let address1 = 0b0000_0000.into();
        let address2 = 0b1111_0000.into();

        let mut cache = FIFOCache::new(16, 16);

        assert_eq!(CacheFetch::Miss, cache.fetch(address1));
        assert_eq!(CacheFetch::Miss, cache.fetch(address2));

        assert_eq!(CacheFetch::Hit, cache.fetch(address1));
        assert_eq!(CacheFetch::Hit, cache.fetch(address2));
    }

    #[test]
    fn test_can_evict_when_cache_is_full () {
        let address1 = 0b0000_0000.into();
        let address2 = 0b0011_0000.into();
        let address3 = 0b0110_0000.into();
        let address4 = 0b1100_0000.into();
        let address5 = 0b1111_0000.into();

        let mut cache = FIFOCache::new(4, 16);

        cache.fetch(address1);
        cache.fetch(address2);
        cache.fetch(address3);
        cache.fetch(address4);

        cache.fetch(address5);

        assert_eq!(CacheFetch::Miss, cache.fetch(address1));
    }
}