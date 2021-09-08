pub struct SourceHeader {
    cache_line_size: usize,
    cache_total_lines: usize,
}

impl SourceHeader {
    pub fn new (cache_total_lines: usize, cache_line_size: usize) -> SourceHeader {
        SourceHeader {
            cache_line_size,
            cache_total_lines,
        }
    }

    pub fn get_cache_line_size (&self) -> usize {
        self.cache_line_size
    }

    pub fn get_cache_total_lines (&self) -> usize {
        self.cache_total_lines
    }
}