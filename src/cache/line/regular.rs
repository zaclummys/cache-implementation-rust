pub struct RegularCacheLine {
    valid: bool,
    tag: usize,
}

impl RegularCacheLine {
    pub fn invalid () -> RegularCacheLine {
        RegularCacheLine {
            tag: 0,
            valid: false,
        }
    }

    pub fn valid (tag: usize) -> RegularCacheLine {
        RegularCacheLine {
            tag,
            valid: true,
        }
    }

    pub fn matches (&self, tag: usize) -> bool {
        self.valid && self.tag == tag
    }
}