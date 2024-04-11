use crate::node::EMPTY_REF;
use crate::tree::Tree;

struct StackNode {
    index: u32,
    left: u32,
    right: u32,
}

pub trait Array<T> {
    fn ordered_list(&self) -> Vec<T>;
}

impl<T: Clone + PartialEq + Eq + PartialOrd + Ord> Array<T> for Tree<T> {
    fn ordered_list(&self) -> Vec<T> {
        if self.root == EMPTY_REF {
            return Vec::new();
        }

        let height = self.height();
        let mut result = Vec::with_capacity(1 << height);

        let mut stack = Vec::with_capacity(height);
        let root_node = self.node(self.root);
        stack.push(StackNode { index: self.root, left: root_node.left, right: root_node.right });

        while !stack.is_empty() {
            let last_stack_index = stack.len() - 1;
            let s = &mut stack[last_stack_index];

            if s.left != EMPTY_REF {
                let index = s.left;
                s.left = EMPTY_REF; // to skip next time

                // go down left
                let node = self.node(index);
                let left = node.left;
                let right = node.right;
                stack.push(StackNode { index, left, right });
            } else {
                if s.index != EMPTY_REF {
                    let index = s.index;
                    s.index = EMPTY_REF; // to skip next time

                    // add value
                    result.push(self.node(index).value.clone());
                }

                if s.right != EMPTY_REF {
                    let index = s.right;
                    s.right = EMPTY_REF; // to skip next time

                    // go down right
                    let node = self.node(index);
                    let left = node.left;
                    let right = node.right;
                    stack.push(StackNode { index, left, right });
                } else {
                    // go up
                    stack.pop();
                }
            }
        }

        result
    }
}