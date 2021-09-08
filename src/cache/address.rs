#[derive(Copy, Clone)]
pub struct CacheAddress {
    address: usize
}

impl CacheAddress {
    pub fn new (address: usize) -> CacheAddress {
        CacheAddress {
            address
        }
    }

    pub fn get_direct_location (&self, total_lines: usize, line_size: usize) -> (usize, usize) {
        let index = (self.address >> line_size.trailing_zeros()) % total_lines;
        let tag = self.address >> line_size.trailing_zeros() >> total_lines.trailing_zeros();

        (tag, index)
    }

    pub fn get_associative_location (&self, line_size: usize) -> usize {
        self.address >> line_size.trailing_zeros()
    }
}

impl From<usize> for CacheAddress {
    fn from (address: usize) -> CacheAddress {
        CacheAddress::new(address)
    }
}

#[cfg(test)]
mod tests {
    use super::CacheAddress;

    #[test]
    fn test_can_get_direct_location () {
        let address = CacheAddress::new(0b11001100_11110000_10101010);

        let (tag, index) = address.get_direct_location(256, 256);

        assert_eq!(0b11001100, tag);
        assert_eq!(0b11110000, index);
    }

    #[test]
    fn test_can_get_associative_location () {
        let address = CacheAddress::new(0b11001100_10101010);

        let tag = address.get_associative_location(256);

        assert_eq!(0b11001100, tag);
    }
}