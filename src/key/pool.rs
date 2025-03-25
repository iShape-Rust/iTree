use crate::{Expiration, ExpiredKey};
use crate::key::node::Node;

pub struct Pool<K, E, V> {
    pub(crate) buffer: Vec<Node<K, E, V>>,
    pub(crate) unused: Vec<u32>
}

impl<K: ExpiredKey<E>, E: Expiration, V: Copy> Pool<K, E, V> {
    #[inline(always)]
    pub(crate) fn new(capacity: usize) -> Self {
        let capacity = capacity.max(8);
        let mut store = Self {
            buffer: Vec::with_capacity(capacity),
            unused: Vec::with_capacity(capacity),
        };
        store.reserve(capacity);
        store
    }

    #[inline]
    fn reserve(&mut self, length: usize) {
        debug_assert!(length > 0);
        let n = self.buffer.len() as u32;
        let l = length as u32;
        let node = Node::default();
        let q = n + l - 1;
        self.buffer.reserve(length);
        self.unused.reserve(length);
        for i in 0..l {
            self.buffer.push(node.clone());
            self.unused.push(q - i);
        }
    }

    #[inline(always)]
    pub(crate) fn get_free_index(&mut self) -> u32 {
        if self.unused.is_empty() {
            let extra_capacity = self.unused.capacity() >> 1;
            self.reserve(extra_capacity);
        }
        self.unused.pop().unwrap()
    }


    #[inline(always)]
    pub(crate) fn put_back(&mut self, index: u32) {
        self.unused.push(index)
    }
}