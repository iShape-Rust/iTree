use std::cmp::Ordering;
use crate::ord::entity::Entity;
use crate::ord::sort::SortedCollection;

pub struct SortedList<K, V> {
    pub(super) buffer: Vec<Entity<K, V>>,
}

impl<K: Copy, V: Copy> SortedList<K, V> {
    #[inline(always)]
    pub fn new(capacity: usize) -> Self {
        Self {
            buffer: Vec::with_capacity(capacity)
        }
    }
}

impl<K: Copy + Ord, V: Copy> SortedCollection<K, V> for SortedList<K, V> {
    #[inline]
    fn is_empty(&self) -> bool {
        self.buffer.is_empty()
    }

    #[inline]
    fn insert(&mut self, key: K, val: V) {
        let index = self
            .buffer
            .binary_search_by_key(&key, |e| e.key)
            .unwrap_or_else(|index| index);
        self.buffer.insert(index, Entity::new(key, val));
    }

    #[inline]
    fn delete(&mut self, key: K) {
        if let Ok(index) = self.buffer.binary_search_by_key(&key, |e| e.key) {
            self.buffer.remove(index);
        }
    }

    #[inline]
    fn get_value(&self, key: K) -> Option<V> {
        if let Ok(index) = self.buffer.binary_search_by_key(&key, |e| e.key) {
            Some(unsafe { self.buffer.get_unchecked(index) }.val)
        } else {
            None
        }
    }

    fn first_less(&self, key: K) -> Option<V> {
        match self.buffer.binary_search_by(|e| e.key.cmp(&key)) {
            Ok(index) => Some(unsafe { self.buffer.get_unchecked(index) }.val),
            Err(index) => {
                if index > 0 {
                    Some(unsafe { self.buffer.get_unchecked(index - 1) }.val)
                } else {
                    None
                }
            }
        }
    }

    fn first_less_by<F>(&self, f: F) -> Option<V>
    where
        F: Fn(K) -> Ordering,
    {
        match self.buffer.binary_search_by(|e| f(e.key)) {
            Ok(index) => Some(unsafe { self.buffer.get_unchecked(index) }.val),
            Err(index) => {
                if index > 0 {
                    Some(unsafe { self.buffer.get_unchecked(index - 1) }.val)
                } else {
                    None
                }
            }
        }
    }

    #[inline]
    fn clear(&mut self) {
        self.buffer.clear();
    }
}