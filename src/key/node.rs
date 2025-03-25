use crate::key::entity::Entity;
use crate::{Expiration, ExpiredKey};

pub(crate) const EMPTY_REF: u32 = u32::MAX;

#[derive(PartialEq, Clone, Copy)]
pub(crate) enum Color {
    Red,
    Black,
}

#[derive(Clone, Copy)]
pub(crate) struct Node<K, E, V> {
    pub(crate) parent: u32,
    pub(crate) left: u32,
    pub(crate) right: u32,
    pub(crate) color: Color,
    pub(crate) entity: Entity<K, E, V>,
}

impl<K: ExpiredKey<E>, E: Expiration, V: Copy> Node<K, E, V> {
    #[inline(always)]
    pub(super) fn is_not_expired(&self, time: E) -> bool {
        self.entity.key.expiration() > time
    }
}

impl<K: ExpiredKey<E>, E: Expiration, V: Copy> Default for Node<K, E, V> {
    #[inline]
    fn default() -> Self {
        Self {
            parent: 0,
            left: 0,
            right: 0,
            color: Color::Red,
            entity: unsafe { std::mem::zeroed() },
        }
    }
}
