use crate::{Expiration, ExpiredKey};
use crate::key::list::KeyExpList;
use crate::key::node::EMPTY_REF;
use crate::key::tree::KeyExpTree;

pub trait IntoArray<E, V> {
    fn into_ordered_vec(self, time: E) -> Vec<V>;
}

impl<K: ExpiredKey<E>, E: Expiration, V: Copy> IntoArray<E, V> for KeyExpList<K, E, V> {
    #[inline]
    fn into_ordered_vec(mut self, time: E) -> Vec<V> {
        self.clear_expired(time);
        self.buffer.iter().map(|e|e.val).collect()
    }
}


impl<K: ExpiredKey<E>, E: Expiration, V: Copy> IntoArray<E, V> for KeyExpTree<K, E, V> {
    #[inline]
    fn into_ordered_vec(mut self, time: E) -> Vec<V> {
        self.collect(time)
    }
}

impl<K: ExpiredKey<E>, E: Expiration, V: Copy> KeyExpTree<K, E, V> {

    fn collect(&mut self, time: E) -> Vec<V> {
        let mut result = Vec::new();
        let mut index = self.expire_root(time);
        if index == EMPTY_REF {
            return result;
        }
        let left_minimum = self.find_expired_left_minimum(index, time);

        let mut prev_index = index;
        let mut last_key = if left_minimum == EMPTY_REF {
            let entity = self.node(index).entity;
            result.push(entity.val);
            entity.key
        } else {
            index = left_minimum;
            let entity = self.node(left_minimum).entity;
            result.push(entity.val);
            entity.key
        };

        while index != EMPTY_REF {
            let node = self.node(index);
            debug_assert!(node.is_not_expired(time));

            let node_entity = node.entity;

            if last_key < node_entity.key {
                if self.node(index).left != prev_index {
                    let lt_index = self.find_expired_left_minimum(index, time);
                    if lt_index != EMPTY_REF {
                        prev_index = index;
                        index = lt_index;
                        continue;
                    }
                }

                last_key = node_entity.key;
                result.push(node_entity.val);
            }

            // left is not exist, or last_key >= node.entity.key
            prev_index = index;
            let rt_index = self.expire_right(index, time);
            if rt_index == EMPTY_REF || last_key >= self.node(rt_index).entity.key {
                // go up
                index = self.expire_parent(index, time);
                continue;
            }

            index = rt_index
        }
        result
    }

    #[inline]
    fn find_expired_left_minimum(&mut self, mut index: u32, time: E) -> u32 {
        index = self.expire_left(index, time);
        let mut result = index;
        while index != EMPTY_REF {
            result = index;
            index = self.expire_left(index, time);
        }
        result
    }
}
