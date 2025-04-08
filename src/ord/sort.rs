use std::cmp::Ordering;

pub trait SortedCollection<K, V> {
    fn is_empty(&self) -> bool;
    fn insert(&mut self, key: K, val: V);
    fn delete(&mut self, key: K);
    fn get_value(&self, key: K) -> Option<V>;
    fn first_less(&self, key: K) -> Option<V>;

    fn first_less_by<F>(&self, f: F) -> Option<V>
    where
        F: Fn(K) -> Ordering;

    fn clear(&mut self);
}
