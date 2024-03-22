use crate::node::{Color, EMPTY_INDEX, Node};

pub struct Store<T> {
    pub(super) buffer: Vec<Node<T>>,
    pub(super) unused: Vec<u32>,
    empty: T,
}

impl<T: Clone> Store<T> {
    pub(super) fn new(empty: T, capacity: usize) -> Self {
        let mut store = Self {
            buffer: Vec::with_capacity(capacity),
            unused: Vec::with_capacity(capacity),
            empty,
        };
        store.reserve(capacity);
        store
    }

    fn reserve(&mut self, length: usize) {
        let n = self.buffer.len() as u32;
        let l = length as u32;
        for i in 0..l {
            let node = Node {
                parent: EMPTY_INDEX,
                left: EMPTY_INDEX,
                right: EMPTY_INDEX,
                color: Color::Red,
                value: self.empty.clone(),
            };
            self.buffer.push(node);
            self.unused.push(n + l - i - 1);
        }
    }

    pub fn get_free_index(&mut self) -> u32 {
        if self.unused.is_empty() {
            self.reserve(16);
        }
        self.unused.pop().unwrap()
    }


    pub fn put_back(&mut self, index: u32) {
        self.unused.push(index)
    }
}