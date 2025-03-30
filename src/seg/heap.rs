pub(super) struct Heap32 {}

impl Heap32 {

    pub(super) const SUB_CAPACITY: u32 = 32 - 1;
    pub(super) const POWER: u32 = 32_u32.ilog2();

    #[inline]
    pub(super) fn range_to_mask(start: u32, end: u32) -> u64 {
        debug_assert!(start < 32);
        debug_assert!(end < 32);

        if end - start == 31 {
            return 1
        }

        let i0 = Self::order_to_heap_index(start);
        let i1 = Self::order_to_heap_index(end);
        let mut w = u64::fill(i0, i1);

        let mut m: u64 = 0;
        while w > 0 {
            let mut wi = w;
            w = 0;
            while wi != 0 {
                let bit_index = wi.trailing_zeros() as u64;
                let parent = (bit_index - 1) >> 1;
                let lt = (parent << 1) | 1;
                let rt = lt + 1;

                let is_lt = ((1 << lt) & wi) != 0;
                let is_rt = ((1 << rt) & wi) != 0;

                let clean_mask = !(0b11 << lt);
                wi &= clean_mask; // clean lt and rt

                match (is_lt, is_rt) { // (false, false) is not possible
                    (true, true) => w |= 1 << parent,
                    (true, false) => m |= 1 << lt,
                    (_, _) => m |= 1 << rt,
                }
            }
        }

        m
    }

    #[inline]
    pub(super) fn order_to_heap_index(order: u32) -> u32 {
        order + Self::SUB_CAPACITY
    }
}

trait BitOp {
    fn fill(start: u32, end: u32) -> u64;
}

impl BitOp for u64 {
    #[inline]
    fn fill(start: u32, end: u32) -> u64 {
        ((1u64 << (end - start + 1)) - 1) << start
    }
}

pub(super) struct BitIter {
    value: u64,
}

impl BitIter {
    #[inline]
    pub(super) fn new(value: u64) -> Self {
        Self { value }
    }
}

impl Iterator for BitIter {
    type Item = usize;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.value == 0 {
            return None;
        }
        let pos = self.value.trailing_zeros() as usize;
        self.value &= self.value - 1;
        Some(pos)
    }
}

#[cfg(test)]
mod tests {
    use crate::seg::heap::{BitIter, BitOp, Heap32};

    #[test]
    fn test_00() {
        assert_eq!(u64::fill(0, 2), 0b111);
        assert_eq!(u64::fill(1, 2), 0b110);
        assert_eq!(u64::fill(2, 2), 0b100);
    }

    #[test]
    fn test_01() {
        let m = Heap32::range_to_mask(0, 31);
        let indices: Vec<_> = BitIter::new(m).collect();
        assert_eq!(indices, vec![0]);
    }

    #[test]
    fn test_02() {
        let m = Heap32::range_to_mask(0, 30);
        let indices: Vec<_> = BitIter::new(m).collect();
        assert_eq!(indices, vec![1, 5, 13, 29, 61]);
    }

    #[test]
    fn test_03() {
        let m = Heap32::range_to_mask(30, 31);
        let indices: Vec<_> = BitIter::new(m).collect();
        assert_eq!(indices, vec![30]);
    }

    #[test]
    fn test_04() {
        let m = Heap32::range_to_mask(29, 31);
        let mut indices: Vec<_> = BitIter::new(m).collect();
        indices.sort_unstable();
        assert_eq!(indices, vec![30, 60]);
    }

    #[test]
    fn test_05() {
        let m = Heap32::range_to_mask(15, 16);
        let mut indices: Vec<_> = BitIter::new(m).collect();
        indices.sort_unstable();
        assert_eq!(indices, vec![46, 47]);
    }

    #[test]
    fn test_06() {
        let m = Heap32::range_to_mask(0, 12);
        let mut indices: Vec<_> = BitIter::new(m).collect();
        indices.sort_unstable();
        assert_eq!(indices, vec![3, 9, 43]);
    }

}