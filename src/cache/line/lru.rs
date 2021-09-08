pub struct LRUCacheLine {
    tag: usize,
    age: usize,
    valid: bool,
}

impl LRUCacheLine {
    pub fn invalid () -> LRUCacheLine {
        LRUCacheLine {
            tag: 0,
            age: 0,
            valid: false,
        }
    }

    pub fn valid (tag: usize, age: usize) -> LRUCacheLine {
        LRUCacheLine {
            tag,
            age,
            valid: true,
        }
    }

    pub fn matches (&self, tag: usize) -> bool {
        self.valid && self.tag == tag
    }

    pub fn get_age (&self) -> usize {
        self.age
    }

    pub fn set_age (&mut self, age: usize) {
        self.age = age;
    }
}
