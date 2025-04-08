use crate::ord::entity::Entity;

pub(super) const EMPTY_REF: u32 = u32::MAX;

#[derive(PartialEq, Clone, Copy)]
pub(super) enum Color {
    Red,
    Black,
}

#[derive(Clone, Copy)]
pub(super) struct Node<K, V> {
    pub(super) parent: u32,
    pub(super) left: u32,
    pub(super) right: u32,
    pub(super) color: Color,
    pub(super) entity: Entity<K, V>,
}

impl<K: Copy, V: Copy> Default for Node<K, V> {
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
