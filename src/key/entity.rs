use std::marker::PhantomData;
use crate::{Expiration, ExpiredKey};

#[derive(Clone, Copy)]
pub(crate) struct Entity<K, E, V> {
    pub(crate) key: K,
    pub(crate) val: V,
    phantom_data: PhantomData<E>
}

impl<K: ExpiredKey<E>, E: Expiration, V: Copy> Entity<K, E, V> {
    #[inline]
    pub(crate) fn new(key: K, val: V) -> Self {
        Self { key, val, phantom_data: Default::default() }
    }
}