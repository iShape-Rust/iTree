use crate::seg::chunk::Chunk;
use crate::seg::entity::Entity;
use crate::seg::exp::{SegExpCollection, SegRange};
use crate::seg::heap::{BitIter, Heap32};
use crate::{Expiration, ExpiredVal};
use std::marker::PhantomData;

pub struct SegExpTree<R, E, V> {
    layout: Layout,
    chunks: Vec<Chunk<E, V>>,
    phantom_data: PhantomData<R>,
}

impl<R, E: Expiration, V: ExpiredVal<E>> SegExpTree<R, E, V>
where
    i64: From<R>,
{
    #[inline]
    pub fn new(range: SegRange<R>) -> Option<Self> {
        let end: i64 = range.max.into();
        let start: i64 = range.min.into();
        let layout = Layout::new(start, end)?;
        let count = layout.count();

        Some(Self {
            layout,
            chunks: vec![Chunk::new(); count],
            phantom_data: Default::default(),
        })
    }

    #[inline]
    fn chunk(&self, index: usize) -> &Chunk<E, V> {
        unsafe { self.chunks.get_unchecked(index) }
    }

    #[inline]
    fn chunk_mut(&mut self, index: usize) -> &mut Chunk<E, V> {
        unsafe { self.chunks.get_unchecked_mut(index) }
    }
}

impl<R, E: Expiration, V: ExpiredVal<E>> SegExpCollection<R, E, V> for SegExpTree<R, E, V>
where
    i64: From<R>,
{

    #[inline]
    fn mask(&self, range: SegRange<R>) -> u64 {
        let start = self.layout.index(range.min.into());
        let end = self.layout.index(range.max.into());

        Heap32::range_to_mask(start, end)
    }

    #[inline]
    fn insert_by_mask(&mut self, mask: u64, val: V, time: E) {
        let entity = Entity::new(val, mask);
        for index in BitIter::new(mask) {
            self.chunk_mut(index).insert(entity, time);
        }
    }

    #[inline]
    fn insert_by_range(&mut self, range: SegRange<R>, val: V, time: E) {
        let mask = self.mask(range);
        self.insert_by_mask(mask, val, time);
    }

    type Iter<'a>
        = SegExpTreeIterator<'a, R, E, V>
    where
        R: 'a,
        E: 'a,
        V: 'a;

    #[inline]
    fn iter_by_mask(&mut self, mask: u64, time: E) -> SegExpTreeIterator<R, E, V> {
        SegExpTreeIterator::new(mask, time, self)
    }

    #[inline]
    fn iter_by_range(&mut self, range: SegRange<R>, time: E) -> SegExpTreeIterator<R, E, V> {
        let mask = self.mask(range);
        SegExpTreeIterator::new(mask, time, self)
    }

    #[inline]
    fn clear(&mut self) {
        for chunk in self.chunks.iter_mut() {
            chunk.clear();
        }
    }
}

struct Layout {
    min: i64,
    max: i64,
    scale: u32,
}

impl Layout {
    #[inline]
    fn new(start: i64, end: i64) -> Option<Self> {
        let min = start - 1;
        let max = end + 1;
        let len = (max - min) as usize;
        if len < Heap32::POWER as usize {
            return None;
        }
        let p = (len - 1).ilog2() + 1;
        let scale = p - Heap32::POWER;

        Some(Self { min, max, scale })
    }

    #[inline]
    fn index(&self, value: i64) -> u32 {
        ((value - self.min) >> self.scale) as u32
    }

    #[inline]
    fn count(&self) -> usize {
        let order = self.index(self.max);
        Heap32::order_to_heap_index(order) as usize
    }
}

pub struct SegExpTreeIterator<'a, R, E, V> {
    tree: &'a mut SegExpTree<R, E, V>,
    time: E,
    i0: usize,
    i1: usize,
    mask: u64,
    bit_iter: BitIter,
}

impl<'a, R, E: Expiration, V: ExpiredVal<E>> SegExpTreeIterator<'a, R, E, V>
where
    i64: From<R>,
{
    #[inline]
    fn new(mask: u64, time: E, tree: &'a mut SegExpTree<R, E, V>) -> Self {
        let mut iter = SegExpTreeIterator {
            tree,
            time,
            i0: 0,
            i1: 0,
            mask,
            bit_iter: BitIter::new(mask),
        };

        // Find the first valid chunk
        iter.i0 = iter.find_next_not_empty_chunk();

        iter
    }

    #[inline]
    fn find_next_not_empty_chunk(&mut self) -> usize {
        while let Some(next) = self.bit_iter.next() {
            let chunk = self.tree.chunk_mut(next);
            chunk.clear_expired(self.time);
            if !chunk.is_empty() {
                return next;
            }
        }
        usize::MAX
    }

}

impl<'a, R, E: Expiration, V: ExpiredVal<E>> Iterator for SegExpTreeIterator<'a, R, E, V>
where
    i64: From<R>,
{
    type Item = V;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        while self.i0 < self.tree.chunks.len() {
            let chunk = self.tree.chunk(self.i0);
            while self.i1 < chunk.buffer.len() {
                let item = chunk.entity(self.i1);
                self.i1 += 1;

                // we must return same pair only once,
                let mask_int = item.mask & self.mask;
                let first_index = mask_int.trailing_zeros() as usize;

                // we will return only for first index
                if first_index == self.i0 {
                    return Some(item.val);
                }
            }

            self.i0 = self.find_next_not_empty_chunk();
            self.i1 = 0;
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use crate::ExpiredVal;
    use crate::seg::exp::{SegExpCollection, SegRange};
    use crate::seg::tree::{Layout, SegExpTree};

    #[test]
    fn test_00() {
        let layout = Layout::new(1, 126).unwrap();
        assert_eq!(layout.index(0), 0);
        assert_eq!(layout.index(1), 0);
        assert_eq!(layout.index(2), 0);
        assert_eq!(layout.index(3), 0);
        assert_eq!(layout.index(4), 1);
        assert_eq!(layout.index(5), 1);
        assert_eq!(layout.index(6), 1);
        assert_eq!(layout.index(7), 1);
        assert_eq!(layout.index(8), 2);

        assert_eq!(layout.index(127), 31);
        assert_eq!(layout.index(126), 31);
        assert_eq!(layout.index(125), 31);
        assert_eq!(layout.index(124), 31);
        assert_eq!(layout.index(123), 30);
        assert_eq!(layout.index(122), 30);
        assert_eq!(layout.index(121), 30);
        assert_eq!(layout.index(120), 30);
        assert_eq!(layout.index(119), 29);
    }

    #[test]
    fn test_01() {
        let layout = Layout::new(0, 100).unwrap();
        assert_eq!(layout.index(-1), 0);
        assert_eq!(layout.index(0), 0);
        assert_eq!(layout.index(1), 0);
        assert_eq!(layout.index(2), 0);
        assert_eq!(layout.index(3), 1);
        assert_eq!(layout.index(4), 1);
        assert_eq!(layout.index(5), 1);
        assert_eq!(layout.index(6), 1);
        assert_eq!(layout.index(7), 2);
        assert_eq!(layout.index(8), 2);

        assert_eq!(layout.index(90), 22);
        assert_eq!(layout.index(91), 23);
        assert_eq!(layout.index(92), 23);
        assert_eq!(layout.index(93), 23);
        assert_eq!(layout.index(94), 23);
        assert_eq!(layout.index(95), 24);
        assert_eq!(layout.index(96), 24);
        assert_eq!(layout.index(97), 24);
        assert_eq!(layout.index(98), 24);
        assert_eq!(layout.index(99), 25);
        assert_eq!(layout.index(100), 25);
        assert_eq!(layout.index(101), 25);
    }

    #[derive(Clone, Copy)]
    struct Point {
        x: i32,
        y: i32,
    }

    #[derive(Clone, Copy)]
    struct Segment {
        a: Point,
        b: Point,
    }

    impl Segment {
        fn new(ax: i32, ay: i32, bx: i32, by: i32) -> Self {
            Self {
                a: Point { x: ax, y: ay },
                b: Point { x: bx, y: by },
            }
        }

        fn y_range(&self) -> SegRange<i32> {
            if self.a.y < self.b.y {
                SegRange {
                    min: self.a.y,
                    max: self.b.y,
                }
            } else {
                SegRange {
                    min: self.b.y,
                    max: self.a.y,
                }
            }
        }
    }

    impl ExpiredVal<i32> for Segment {
        fn expiration(&self) -> i32 {
            self.a.x.max(self.b.x)
        }
    }

    #[test]
    fn test_02() {
        let mut tree = SegExpTree::new(SegRange { min: 0, max: 128 }).unwrap();
        let s = Segment::new(0, 2, 2, 100);
        tree.insert_by_range(s.y_range(), s, 0);
        tree.clear();
        for chunk in tree.chunks {
            assert!(chunk.is_empty());
        }
    }

    #[test]
    fn test_03() {
        let mut tree = SegExpTree::new(SegRange { min: 0, max: 128 }).unwrap();
        let s = Segment::new(0, 2, 2, 100);
        tree.insert_by_range(s.y_range(), s, 0);
        let mut result = Vec::new();
        for val in tree.iter_by_range(SegRange { min: 0, max: 100 }, 0) {
            result.push(val);
        }
        assert_eq!(result.len(), 1);
    }

    #[test]
    fn test_04() {
        let mut tree = SegExpTree::new(SegRange { min: 0, max: 128 }).unwrap();
        let s0 = Segment::new(0, 10, 2, 100);
        let s1 = Segment::new(0, 20, 2, 80);

        tree.insert_by_range(s0.y_range(), s0, 0);
        tree.insert_by_range(s1.y_range(), s1, 0);

        let mut result = Vec::new();
        for val in tree.iter_by_range(SegRange { min: 15, max: 90 }, 0) {
            result.push(val);
        }
        assert_eq!(result.len(), 2);
    }

    #[test]
    fn test_05() {
        let mut tree = SegExpTree::new(SegRange { min: 0, max: 128 }).unwrap();
        let s0 = Segment::new(0, 10, 2, 20);
        let s1 = Segment::new(0, 80, 2, 100);

        tree.insert_by_range(s0.y_range(), s0, 0);
        tree.insert_by_range(s1.y_range(), s1, 0);

        let mut result = Vec::new();
        for val in tree.iter_by_range(SegRange { min: 40, max: 60 }, 0) {
            result.push(val);
        }
        assert_eq!(result.len(), 0);
    }
}
