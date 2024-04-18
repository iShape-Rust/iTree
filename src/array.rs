use crate::node::EMPTY_REF;
use crate::tree::Tree;

pub struct StackNode {
    index: u32,
    left: u32,
    right: u32,
}

pub trait Array<T> {
    fn ordered_list(&self) -> Vec<T>;
    fn fill_ordered_list(&self, list: &mut Vec<T>);
    fn fill_ordered_list_with_stack(&self, list: &mut Vec<T>, stack: &mut Vec<StackNode>);
}

impl<T: Clone + PartialEq + Eq + PartialOrd + Ord> Array<T> for Tree<T> {
    fn ordered_list(&self) -> Vec<T> {
        if self.root == EMPTY_REF {
            return Vec::new();
        }

        let height = self.height();
        let mut list = Vec::with_capacity(1 << height);
        let mut stack = Vec::with_capacity(height);

        self.fill_ordered_list_with_stack(&mut list, &mut stack);

        list
    }

    fn fill_ordered_list(&self, list: &mut Vec<T>) {
        let height = self.height();
        let mut stack = Vec::with_capacity(height);
        self.fill_ordered_list_with_stack(list, &mut stack);
    }

    fn fill_ordered_list_with_stack(&self, list: &mut Vec<T>, stack: &mut Vec<StackNode>) {
        list.clear();
        stack.clear();

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
                    list.push(self.node(index).value.clone());
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
    }
}