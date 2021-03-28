#[derive(PartialEq, Debug, Clone)]
pub struct BitSet64(u64);

impl BitSet64 {
    pub fn new(value: u64) -> Self {
        BitSet64(value)
    }

    pub fn new_filled(set_bits: u8) -> Self {
        let mut result = BitSet64(0);
        for set_bit in 0..set_bits {
            result.set(set_bit);
        }
        result
    }

    pub fn and(&self, other: &Self) -> Self {
        BitSet64(self.0 & other.0)
    }

    pub fn or(&self, other: &Self) -> Self {
        BitSet64(self.0 | other.0)
    }

    pub fn set(&mut self, bit: u8) {
        self.0 |= 1 << bit;
    }

    pub fn any(&self) -> bool {
        self.0 != 0
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_filled() {
        assert_eq!(BitSet64::new_filled(0), BitSet64(0b0));
        assert_eq!(BitSet64::new_filled(1), BitSet64(0b1));
        assert_eq!(BitSet64::new_filled(2), BitSet64(0b11));
        assert_eq!(BitSet64::new_filled(3), BitSet64(0b111));
        assert_eq!(BitSet64::new_filled(64), BitSet64(0xffff_ffff_ffff_ffff));
    }

    #[test]
    fn test_or() {
        assert_eq!(BitSet64::new(0b011).or(&BitSet64(0b101)), BitSet64(0b111));
    }

    #[test]
    fn test_and() {
        assert_eq!(BitSet64::new(0b011).and(&BitSet64(0b110)), BitSet64(0b10));
    }

    #[test]
    fn test_any() {
        assert_eq!(BitSet64::new(0b0).any(), false);
        assert_eq!(BitSet64::new(0b1).any(), true);
        assert_eq!(BitSet64::new(0b10).any(), true);
    }

    #[test]
    fn test_set() {
        let mut bitset = BitSet64::new(0);
        bitset.set(0);
        assert_eq!(bitset, BitSet64(0b1));

        bitset.set(1);
        assert_eq!(bitset, BitSet64(0b11));

        bitset.set(63);
        assert_eq!(bitset, BitSet64(0x8000_0000_0000_0003));
    }
}