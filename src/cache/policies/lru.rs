use crate::cache::{
    Cache,
    CacheFetch,
    CacheAddress,
    LRUCacheLine,
};

pub struct LRUCache {
    age: usize,
    line_size: usize,
    lines: Vec<LRUCacheLine>,
}

impl LRUCache {
    pub fn new (total_lines: usize, line_size: usize) -> LRUCache {
        let mut lines = Vec::with_capacity(total_lines);

        for _ in 0..total_lines {
            lines.push(LRUCacheLine::invalid());
        }

        LRUCache {
            age: 0,
            lines,
            line_size,
        }
    }

    fn tick (&mut self) {
        self.age += 1;
    }

    fn find (&mut self, tag: usize) -> Option<usize> {
        self.lines.iter_mut().position(|line| line.matches(tag))
    }

    fn insert (&mut self, tag: usize) {
        let age = self.age;

        let mut min_line_age = age;
        let mut min_line_age_index = 0;

        for (index, line) in self.lines.iter().enumerate() {
            let line_age = line.get_age();

            if line_age < min_line_age {
                min_line_age = line_age;
                min_line_age_index = index;
            }
        }

        self.lines[min_line_age_index] = LRUCacheLine::valid(tag, age);
    }

    fn refresh (&mut self, cache_line_index: usize) {
        self.lines[cache_line_index].set_age(self.age);
    }
}

impl Cache for LRUCache {
    fn fetch (&mut self, address: CacheAddress) -> CacheFetch {
        self.tick();

        let tag = address.get_associative_location(self.line_size);

        match self.find(tag) {
            Some (index) => {
                self.refresh(index);

                CacheFetch::Hit
            }

            None => {
                self.insert(tag);

                CacheFetch::Miss
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Cache, CacheFetch, LRUCache};

    #[test]
    fn test_can_hit_after_miss() {
        let address1 = 0b0000_0000.into();
        let address2 = 0b1111_0000.into();

        let mut cache = LRUCache::new(16, 16);

        assert_eq!(CacheFetch::Miss, cache.fetch(address1));
        assert_eq!(CacheFetch::Miss, cache.fetch(address2));

        assert_eq!(CacheFetch::Hit, cache.fetch(address1));
        assert_eq!(CacheFetch::Hit, cache.fetch(address2));
    }

    #[test]
    fn test_can_evict_when_cache_is_full() {
        let address1 = 0b0000_0000.into();
        let address2 = 0b0011_0000.into();
        let address3 = 0b0110_0000.into();
        let address4 = 0b1100_0000.into();
        let address5 = 0b1111_0000.into();

        let mut cache = LRUCache::new(4, 16);

        cache.fetch(address1);
        cache.fetch(address2);
        cache.fetch(address3);
        cache.fetch(address4);

        cache.fetch(address1);
        cache.fetch(address2);
        cache.fetch(address3);

        cache.fetch(address5);

        assert_eq!(CacheFetch::Miss, cache.fetch(address4));
    }
}
