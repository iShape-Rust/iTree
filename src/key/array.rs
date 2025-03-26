use crate::{Expiration, ExpiredKey};
use crate::key::list::KeyExpList;
use crate::key::node::{Color, Node, EMPTY_REF};
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
        // self.create_ordered_list(time)
        self.collect(time)
    }
}

struct StackNode {
    index: u32,
    left: u32,
    right: u32
}

impl StackNode {
    fn new<K, E, V>(index: u32, node: &Node<K, E, V>) -> Self {
        Self {
            index,
            left: node.left,
            right: node.right,
        }
    }
}

impl<K: ExpiredKey<E>, E: Expiration, V: Copy> KeyExpTree<K, E, V> {

    #[inline]
    fn create_ordered_list(&mut self, time: E) -> Vec<V> {
        self.expire_all(time);

        let height = self.height();
        let mut stack = Vec::with_capacity(height);
        let mut list = Vec::with_capacity(8 << height);

        if self.root == EMPTY_REF {
            return list;
        }

        stack.push(StackNode::new(self.root, self.node(self.root)));

        while !stack.is_empty() {
            let last_stack_index = stack.len() - 1;
            let s = &mut stack[last_stack_index];

            if s.left != EMPTY_REF {
                // go down left
                let index = s.left;
                // to skip next time
                s.left = EMPTY_REF;

                stack.push(StackNode::new(index, self.node(index)));
            } else {
                if s.index != EMPTY_REF {
                    let index = s.index;
                    // to skip next time
                    s.index = EMPTY_REF;

                    let node = self.node(index);

                    list.push(node.entity.val);
                }

                if s.right != EMPTY_REF {
                    // go down right
                    let index = s.right;
                    // to skip next time
                    s.right = EMPTY_REF;

                    stack.push(StackNode::new(index, self.node(index)));
                } else {
                    // go up
                    stack.pop();
                }
            }
        }

        list
    }

    #[inline]
    fn height(&self) -> usize {
        if self.root == EMPTY_REF { return 0; }
        let mut node = self.node(self.root);
        let mut height = 1;
        while node.left != EMPTY_REF {
            node = self.node(node.left);
            if node.color == Color::Black {
                height += 1;
            }
        }

        height << 1
    }

    #[inline]
    fn expire_all(&mut self, time: E) {
        let n = self.store.buffer.len() as u32;
        for i in 1..n {
            if self.is_part_of_the_tree(i) && self.node(i).entity.key.expiration() < time {
                    self.delete_index(i);
            }
        }
    }

    #[inline]
    fn is_part_of_the_tree(&self, index: u32) -> bool {
        let mut prev = index;
        let mut cursor = self.node(index).parent;
        while cursor != 0 && cursor != EMPTY_REF && cursor != index {
            prev = cursor;
            let parent_index = self.node(cursor).parent;
            if parent_index == EMPTY_REF {
                break;
            }
            let parent = self.node(parent_index);
            if parent.left != cursor && parent.right != cursor {
                return false;
            }
            cursor = parent_index;
        }
        prev == self.root
    }


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
