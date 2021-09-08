use crate::cache::{
    Cache,
    CacheFetch,
    CacheAddress,
    RegularCacheLine,
};

pub struct DirectCache {
    line_size: usize,
    total_lines: usize,
    lines: Vec<RegularCacheLine>,
}

impl DirectCache {
    pub fn new (total_lines: usize, line_size: usize) -> DirectCache {
        let mut lines = Vec::with_capacity(total_lines);

        for _ in 0..total_lines {
            lines.push(RegularCacheLine::invalid());
        }

        DirectCache {
            lines,
            line_size,
            total_lines,
        }
    }

    fn exists (&self, tag: usize, index: usize) -> bool {
        self.lines[index].matches(tag)
    }

    fn insert (&mut self, tag: usize, index: usize) {
        self.lines[index] = RegularCacheLine::valid(tag);
    }
}

impl Cache for DirectCache {
    fn fetch (&mut self, address: CacheAddress) -> CacheFetch {
        let (tag, index) = address.get_direct_location(
            self.total_lines,
            self.line_size,
        );

        if self.exists(tag, index) {
            CacheFetch::Hit
        }
        else {
            self.insert(tag, index);

            CacheFetch::Miss
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Cache, CacheFetch, DirectCache};

    #[test]
    fn test_can_hit_after_miss () {
        let address = 0b0000_0010_0000.into();

        let mut cache = DirectCache::new(16, 16);

        assert_eq!(CacheFetch::Miss, cache.fetch(address));
        assert_eq!(CacheFetch::Hit, cache.fetch(address));
    }

    #[test]
    fn test_can_hit_after_miss_with_same_tag_but_different_line_addresses () {
        let address1 = 0b0000_0000_0000.into();
        let address2 = 0b0000_1111_0000.into();

        let mut cache = DirectCache::new(16, 16);

        assert_eq!(CacheFetch::Miss, cache.fetch(address1));
        assert_eq!(CacheFetch::Miss, cache.fetch(address2));

        assert_eq!(CacheFetch::Hit, cache.fetch(address1));
        assert_eq!(CacheFetch::Hit, cache.fetch(address2));
    }

    #[test]
    fn test_can_evict_with_different_tag_but_same_line_addresses () {
        let address1 = 0b0000_0000_0000.into();
        let address2 = 0b1111_0000_0000.into();

        let mut cache = DirectCache::new(16, 16);

        assert_eq!(CacheFetch::Miss, cache.fetch(address1));
        assert_eq!(CacheFetch::Miss, cache.fetch(address2));

        assert_eq!(CacheFetch::Miss, cache.fetch(address1));
        assert_eq!(CacheFetch::Miss, cache.fetch(address2));
    }
}