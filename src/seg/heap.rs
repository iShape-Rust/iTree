use crate::seg::bit::BitOp;

pub(super) struct Heap32 {}

impl Heap32 {
    pub(super) const SUB_CAPACITY: u32 = 32 - 1;
    pub(super) const POWER: u32 = 32_u32.ilog2();

    #[inline]
    pub(super) fn range_to_intersect_mask(start: u32, end: u32) -> u64 {
        debug_assert!(start <= end);
        debug_assert!(end < 32);

        // Intersected nodes form one contiguous interval on each heap level.
        let mut mask = 0;
        for level in 0..=Self::POWER {
            let offset = Self::SUB_CAPACITY >> level;
            mask |= u64::fill(offset + (start >> level), offset + (end >> level));
        }
        mask
    }

    #[inline]
    pub(super) fn range_to_place_mask(start: u32, end: u32) -> u64 {
        debug_assert!(start <= end);
        debug_assert!(end < 32);

        if end - start == 31 {
            return 1;
        }

        // Use one-based heap indices while merging complete sibling pairs.
        // Unpaired boundary nodes belong to the minimal cover of the range.
        let mut left = start + 32;
        let mut right = end + 32;
        let mut mask = 0;
        while left <= right {
            if left & 1 != 0 {
                mask |= 1_u64 << (left - 1);
                left += 1;
            }
            if right & 1 == 0 {
                mask |= 1_u64 << (right - 1);
                right -= 1;
            }
            left >>= 1;
            right >>= 1;
        }
        mask
    }

    #[inline]
    pub(super) fn order_to_heap_index(order: u32) -> u32 {
        order + Self::SUB_CAPACITY
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
    use crate::seg::heap::{BitIter, Heap32};
    use alloc::vec;
    use alloc::vec::Vec;

    // Independent reference: recursively test the geometric interval of each node.
    fn reference_masks(start: u32, end: u32, lo: u32, hi: u32, node: u32) -> (u64, u64) {
        if end < lo || hi < start {
            return (0, 0);
        }
        let bit = 1_u64 << node;
        if lo == hi {
            return (bit, bit);
        }
        let mid = (lo + hi) / 2;
        let (li, lp) = reference_masks(start, end, lo, mid, 2 * node + 1);
        let (ri, rp) = reference_masks(start, end, mid + 1, hi, 2 * node + 2);
        let place = if start <= lo && hi <= end { bit } else { lp | rp };
        (bit | li | ri, place)
    }

    #[test]
    fn masks_match_interval_coverage_for_all_ranges() {
        for start in 0..32 {
            for end in start..32 {
                let (intersect, place) = reference_masks(start, end, 0, 31, 0);
                assert_eq!(
                    Heap32::range_to_intersect_mask(start, end),
                    intersect,
                    "intersection mask for [{start}, {end}]"
                );
                assert_eq!(
                    Heap32::range_to_place_mask(start, end),
                    place,
                    "placement mask for [{start}, {end}]"
                );
            }
        }
    }

    #[test]
    fn masks_detect_intersection_for_all_range_pairs() {
        let ranges: Vec<_> = (0..32)
            .flat_map(|start| {
                (start..32).map(move |end| {
                    (
                        start,
                        end,
                        Heap32::range_to_place_mask(start, end),
                        Heap32::range_to_intersect_mask(start, end),
                    )
                })
            })
            .collect();
        for &(a, b, place, _) in &ranges {
            for &(c, d, _, intersect) in &ranges {
                assert_eq!(
                    place & intersect != 0,
                    a <= d && c <= b,
                    "[{a}, {b}] against [{c}, {d}]"
                );
            }
        }
    }

    #[test]
    fn test_00() {
        let m = Heap32::range_to_place_mask(0, 31);
        let indices: Vec<_> = BitIter::new(m).collect();
        assert_eq!(indices, vec![0]);
    }

    #[test]
    fn test_01() {
        let m = Heap32::range_to_place_mask(0, 30);
        let indices: Vec<_> = BitIter::new(m).collect();
        assert_eq!(indices, vec![1, 5, 13, 29, 61]);
    }

    #[test]
    fn test_02() {
        let m = Heap32::range_to_place_mask(30, 31);
        let indices: Vec<_> = BitIter::new(m).collect();
        assert_eq!(indices, vec![30]);
    }

    #[test]
    fn test_03() {
        let m = Heap32::range_to_place_mask(29, 31);
        let mut indices: Vec<_> = BitIter::new(m).collect();
        indices.sort_unstable();
        assert_eq!(indices, vec![30, 60]);
    }

    #[test]
    fn test_04() {
        let m = Heap32::range_to_place_mask(15, 16);
        let mut indices: Vec<_> = BitIter::new(m).collect();
        indices.sort_unstable();
        assert_eq!(indices, vec![46, 47]);
    }

    #[test]
    fn test_05() {
        let m = Heap32::range_to_place_mask(0, 12);
        let mut indices: Vec<_> = BitIter::new(m).collect();
        indices.sort_unstable();
        assert_eq!(indices, vec![3, 9, 43]);
    }

    #[test]
    fn test_06() {
        let m = Heap32::range_to_intersect_mask(0, 0);
        let mut indices: Vec<_> = BitIter::new(m).collect();
        indices.sort_unstable();
        assert_eq!(indices, vec![0, 1, 3, 7, 15, 31]);
    }

    #[test]
    fn test_07() {
        let m = Heap32::range_to_intersect_mask(1, 1);
        let mut indices: Vec<_> = BitIter::new(m).collect();
        indices.sort_unstable();
        assert_eq!(indices, vec![0, 1, 3, 7, 15, 32]);
    }

    #[test]
    fn test_08() {
        let m = Heap32::range_to_intersect_mask(0, 1);
        let mut indices: Vec<_> = BitIter::new(m).collect();
        indices.sort_unstable();
        assert_eq!(indices, vec![0, 1, 3, 7, 15, 31, 32]);
    }

    #[test]
    fn test_09() {
        let m = Heap32::range_to_intersect_mask(0, 31);
        let mut indices: Vec<_> = BitIter::new(m).collect();
        indices.sort_unstable();
        let template: Vec<_> = (0..63).collect();
        assert_eq!(indices, template);
    }
}
