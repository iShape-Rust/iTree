use crate::node::{Color, EMPTY_REF, Node};
use crate::store::Store;

pub struct Tree<T> {
    pub store: Store<T>,
    pub root: u32,
    nil_index: u32,
}

impl<T: Clone + PartialEq + Eq + PartialOrd + Ord> Tree<T> {
    pub fn is_empty(&self) -> bool {
        self.root == EMPTY_REF
    }

    pub fn new(empty: T, capacity: usize) -> Self {
        let mut store = Store::new(empty, capacity);
        let nil_index = store.get_free_index();
        Self {
            store,
            root: EMPTY_REF,
            nil_index,
        }
    }

    pub fn clear_all(&mut self) {
        if self.root == EMPTY_REF {
            return;
        }
        self.store.put_back(self.root);
        self.root = EMPTY_REF;

        let mut n = 1;
        while n > 0 {
            let i0 = self.store.unused.len() - n;
            n = 0;
            for i in i0..self.store.unused.len() {
                let index = self.store.unused[i];
                let node = self.node(index);
                let left = node.left;
                let right = node.right;
                if left != EMPTY_REF {
                    self.store.put_back(left);
                    n += 1;
                }
                if right != EMPTY_REF {
                    self.store.put_back(right);
                    n += 1;
                }
            }
        }
    }

    pub fn is_black(&self, index: u32) -> bool {
        index == EMPTY_REF || index != EMPTY_REF && self.node(index).color == Color::Black
    }

    pub fn node(&self, index: u32) -> &Node<T> {
        unsafe {
            self.store.buffer.get_unchecked(index as usize)
        }
    }

    pub fn mut_node(&mut self, index: u32) -> &mut Node<T> {
        unsafe {
            self.store.buffer.get_unchecked_mut(index as usize)
        }
    }

    fn create_nil_node(&mut self, parent: u32) {
        let node = self.mut_node(self.nil_index);
        node.parent = parent;
        node.left = EMPTY_REF;
        node.right = EMPTY_REF;
        node.color = Color::Red;
    }

    fn rotate_right(&mut self, index: u32) {
        let n = self.node(index);
        let p = n.parent;

        let lt_index = n.left;
        let lt_right = self.node(lt_index).right;

        if lt_right != EMPTY_REF {
            self.mut_node(lt_right).parent = index;
            self.mut_node(index).left = lt_right;
        } else {
            self.mut_node(index).left = EMPTY_REF;
        }

        self.mut_node(index).parent = lt_index;
        self.mut_node(lt_index).right = index;

        self.replace_parents_child(p, index, lt_index);
    }

    fn rotate_left(&mut self, index: u32) {
        let n = self.node(index);
        let p = n.parent;

        let rt_index = n.right;
        let rt_left = self.node(rt_index).left;

        if rt_left != EMPTY_REF {
            self.mut_node(rt_left).parent = index;
            self.mut_node(index).right = rt_left;
        } else {
            self.mut_node(index).right = EMPTY_REF;
        }

        self.mut_node(index).parent = rt_index;
        self.mut_node(rt_index).left = index;

        self.replace_parents_child(p, index, rt_index);
    }

    fn replace_parents_child(&mut self, parent: u32, old_child: u32, new_child: u32) {
        self.mut_node(new_child).parent = parent;
        if parent == EMPTY_REF {
            self.root = new_child;
            return;
        }

        let p = self.mut_node(parent);
        assert!(p.left == old_child || p.right == old_child, "Node is not a child of its parent");

        if p.left == old_child {
            p.left = new_child;
        } else {
            p.right = new_child;
        }
    }

    fn remove_parents_child(&mut self, parent: u32, old_child: u32) {
        let p = self.mut_node(parent);
        assert!(p.left == old_child || p.right == old_child, "Node is not a child of its parent");

        if p.left == old_child {
            p.left = EMPTY_REF;
        } else {
            p.right = EMPTY_REF;
        }
    }

    pub fn insert_if_not_exist(&mut self, value: T) -> bool {
        if self.root == EMPTY_REF {
            self.insert_root(value);
            return true;
        }

        let mut index = self.root;
        let mut p_index = self.root;
        let mut is_left = false;

        while index != EMPTY_REF {
            let node = self.node(index);
            p_index = index;
            if node.value == value {
                return false;
            }

            is_left = value < node.value;
            if is_left {
                is_left = true;
                index = node.left;
            } else {
                index = node.right;
            }
        }

        _ = self.insert_with_parent(value, p_index, is_left);

        true
    }

    pub fn insert(&mut self, value: T) {
        if self.root == EMPTY_REF {
            self.insert_root(value);
            return;
        }

        let mut index = self.root;
        let mut p_index = self.root;
        let mut is_left = false;

        while index != EMPTY_REF {
            let node = self.node(index);
            p_index = index;
            assert!(node.value != value);

            is_left = value < node.value;
            if is_left {
                is_left = true;
                index = node.left;
            } else {
                index = node.right;
            }
        }

        _ = self.insert_with_parent(value, p_index, is_left);
    }

    pub fn insert_root(&mut self, value: T) {
        let new_index = self.store.get_free_index();
        let new_node = self.mut_node(new_index);
        new_node.parent = EMPTY_REF;
        new_node.left = EMPTY_REF;
        new_node.right = EMPTY_REF;
        new_node.color = Color::Black;
        new_node.value = value;
        self.root = new_index;
    }

    pub fn insert_with_parent(&mut self, value: T, p_index: u32, is_left: bool) -> u32 {
        let new_index = self.store.get_free_index();
        let new_node = self.mut_node(new_index);
        new_node.parent = p_index;
        new_node.left = EMPTY_REF;
        new_node.right = EMPTY_REF;
        new_node.color = Color::Red;
        new_node.value = value;

        let parent = self.mut_node(p_index);

        if is_left {
            parent.left = new_index;
        } else {
            parent.right = new_index;
        }

        if parent.color == Color::Red {
            self.fix_red_black_properties_after_insert(new_index, p_index);
        }

        new_index
    }

    pub fn fix_red_black_properties_after_insert(&mut self, n_index: u32, p_origin: u32) {
        // parent is red!
        let mut p_index = p_origin;
        // Case 2:
        // Not having a grandparent means that parent is the root. If we enforce black roots
        // (rule 2), grandparent will never be null, and the following if-then block can be
        // removed.
        let g_index = self.node(p_index).parent;
        if g_index == EMPTY_REF {
            // As this method is only called on red nodes (either on newly inserted ones - or -
            // recursively on red grandparents), all we have to do is to recolor the root black.
            self.mut_node(p_index).color = Color::Black;
            return;
        }

        // Case 3: Uncle is red -> recolor parent, grandparent and uncle
        let u_index = self.get_uncle(p_index);

        if u_index != EMPTY_REF && self.node(u_index).color == Color::Red {
            self.mut_node(p_index).color = Color::Black;
            self.mut_node(g_index).color = Color::Red;
            self.mut_node(u_index).color = Color::Black;

            // Call recursively for grandparent, which is now red.
            // It might be root or have a red parent, in which case we need to fix more...
            let gg_index = self.node(g_index).parent;
            if gg_index != EMPTY_REF && self.node(gg_index).color == Color::Red {
                self.fix_red_black_properties_after_insert(g_index, gg_index);
            }
        } else if p_index == self.node(g_index).left {
            // Parent is left child of grandparent
            // Case 4a: Uncle is black and node is left->right "inner child" of its grandparent
            if n_index == self.node(p_index).right {
                self.rotate_left(p_index);

                // Let "parent" point to the new root node of the rotated subtree.
                // It will be recolored in the next step, which we're going to fall-through to.
                p_index = n_index;
            }

            // Case 5a: Uncle is black and node is left->left "outer child" of its grandparent
            self.rotate_right(g_index);

            // Recolor original parent and grandparent
            self.mut_node(p_index).color = Color::Black;
            self.mut_node(g_index).color = Color::Red;
        } else {
            // Parent is right child of grandparent
            // Case 4b: Uncle is black and node is right->left "inner child" of its grandparent
            if n_index == self.node(p_index).left {
                self.rotate_right(p_index);

                // Let "parent" point to the new root node of the rotated subtree.
                // It will be recolored in the next step, which we're going to fall-through to.
                p_index = n_index;
            }

            // Case 5b: Uncle is black and node is right->right "outer child" of its grandparent
            self.rotate_left(g_index);

            // Recolor original parent and grandparent
            self.mut_node(p_index).color = Color::Black;
            self.mut_node(g_index).color = Color::Red;
        }
    }

    fn get_uncle(&self, p_index: u32) -> u32 {
        let parent = self.node(p_index);
        if parent.parent == EMPTY_REF {
            return EMPTY_REF;
        }

        let grandparent = self.node(parent.parent);

        assert!(grandparent.left == p_index || grandparent.right == p_index, "Parent is not a child of its grandparent");

        if grandparent.left == p_index {
            grandparent.right
        } else {
            grandparent.left
        }
    }

    pub fn delete(&mut self, value: T) {
        let mut index = self.root;
        // Find the node to be deleted
        while index != EMPTY_REF {
            let node = self.node(index);
            if value == node.value {
                break;
            } else if value < node.value {
                index = node.left;
            } else {
                index = node.right;
            }
        }

        if index == EMPTY_REF {
            debug_assert!(false, "value is not found");
            return;
        }

        _ = self.delete_index(index);
    }

    pub fn delete_if_exist(&mut self, value: T) {
        let mut index = self.root;
        // Find the node to be deleted
        while index != EMPTY_REF {
            let node = self.node(index);
            if value == node.value {
                _ = self.delete_index(index);
                return;
            } else if value < node.value {
                index = node.left;
            } else {
                index = node.right;
            }
        }
    }

    pub fn delete_index(&mut self, index: u32) -> u32 {
        let moved_up_node: u32;
        let deleted_node_color: Color;

        let node = self.node(index);

        let is_root = index == self.root;
        let is_single = node.left == EMPTY_REF || node.right == EMPTY_REF;

        // Node has zero or one child
        if is_single {
            deleted_node_color = node.color;
            moved_up_node = self.delete_node_with_zero_or_one_child(index);
        } else {
            let successor_index = self.find_left_minimum(node.right);
            let successor = self.node(successor_index);
            deleted_node_color = successor.color;

            self.mut_node(index).value = successor.value.clone();

            moved_up_node = self.delete_node_with_zero_or_one_child(successor_index);
        }

        if moved_up_node == EMPTY_REF || deleted_node_color != Color::Black {
            return if is_single {
                self.parent(index)
            } else if is_root {
                self.root
            } else {
                index
            };
        }

        self.fix_red_black_properties_after_delete(moved_up_node);

        if moved_up_node == self.nil_index {
            let p_index = self.node(moved_up_node).parent;

            if p_index != EMPTY_REF {
                self.remove_parents_child(p_index, moved_up_node);
            }
        }

        if is_single {
            self.parent(index)
        } else if is_root {
            self.root
        } else {
            index
        }
    }

    fn parent(&self, index: u32) -> u32 {
        let parent = self.node(index).parent;
        if parent == EMPTY_REF {
            self.root
        } else {
            parent
        }
    }

    fn delete_node_with_zero_or_one_child(&mut self, n_index: u32) -> u32 {
        self.store.put_back(n_index);
        let node = self.node(n_index);
        let nd_left = node.left;
        let nd_right = node.right;
        let nd_parent = node.parent;
        let nd_color = node.color;

        if nd_left != EMPTY_REF {
            // Node has ONLY a left child --> replace by its left child
            self.replace_parents_child(nd_parent, n_index, nd_left);
            nd_left // moved-up node
        } else if nd_right != EMPTY_REF {
            // Node has ONLY a right child --> replace by its right child
            self.replace_parents_child(nd_parent, n_index, nd_right);
            nd_right // moved-up node
        } else {
            // Node has no children -->
            // * node is red --> just remove it
            // * node is black --> replace it by a temporary NIL node (needed to fix the R-B rules)
            if nd_parent != EMPTY_REF {
                if nd_color == Color::Black {
                    self.create_nil_node(nd_parent);
                    self.replace_parents_child(nd_parent, n_index, self.nil_index);
                    self.nil_index
                } else {
                    self.remove_parents_child(nd_parent, n_index);
                    EMPTY_REF
                }
            } else {
                self.root = EMPTY_REF;
                EMPTY_REF
            }
        }
    }

    fn fix_red_black_properties_after_delete(&mut self, n_index: u32) {
        // Case 1: Examined node is root, end of recursion
        if n_index == self.root {
            // do not color root to black
            return;
        }

        let mut s_index = self.get_sibling(n_index);

        // Case 2: Red sibling
        if self.node(s_index).color == Color::Red {
            self.handle_red_sibling(n_index, s_index);
            s_index = self.get_sibling(n_index) // Get new sibling for fall-through to cases 3-6
        }

        let sibling = self.node(s_index);

        // Cases 3+4: Black sibling with two black children
        if self.is_black(sibling.left) && self.is_black(sibling.right) {
            self.mut_node(s_index).color = Color::Red;
            let p_index = self.node(n_index).parent;

            // Case 3: Black sibling with two black children + red parent
            let parent = self.mut_node(p_index);
            if parent.color == Color::Red {
                parent.color = Color::Black;
            } else {
                // Case 4: Black sibling with two black children + black parent
                self.fix_red_black_properties_after_delete(p_index);
            }
        } else {
            // Case 5+6: Black sibling with at least one red child
            self.handle_black_sibling_with_at_least_one_red_child(n_index, s_index);
        }
    }

    fn handle_black_sibling_with_at_least_one_red_child(&mut self, n_index: u32, s_origin: u32) {
        let p_index = self.node(n_index).parent;

        let mut s_index = s_origin;
        let (mut sibling_left, mut sibling_right) = {
            let sibling = self.node(s_origin);
            (sibling.left, sibling.right)
        };

        let node_is_left_child = n_index == self.node(p_index).left;

        // Case 5: Black sibling with at least one red child + "outer nephew" is black
        // --> Recolor sibling and its child, and rotate around sibling
        if node_is_left_child && self.is_black(sibling_right) {
            if sibling_left != EMPTY_REF {
                self.mut_node(sibling_left).color = Color::Black;
            }
            self.mut_node(s_index).color = Color::Red;
            self.rotate_right(s_index);
            s_index = self.node(p_index).right;

            let sibling = self.node(s_index);
            sibling_left = sibling.left;
            sibling_right = sibling.right;
        } else if !node_is_left_child && self.is_black(sibling_left) {
            if sibling_right != EMPTY_REF {
                self.mut_node(sibling_right).color = Color::Black;
            }
            self.mut_node(s_index).color = Color::Red;
            self.rotate_left(s_index);
            s_index = self.node(p_index).left;

            let sibling = self.node(s_index);
            sibling_left = sibling.left;
            sibling_right = sibling.right;
        }

        // Fall-through to case 6...

        // Case 6: Black sibling with at least one red child + "outer nephew" is red
        // --> Recolor sibling + parent + sibling's child, and rotate around parent
        self.mut_node(s_index).color = self.node(p_index).color;
        self.mut_node(p_index).color = Color::Black;
        if node_is_left_child {
            if sibling_right != EMPTY_REF {
                self.mut_node(sibling_right).color = Color::Black;
            }
            self.rotate_left(p_index)
        } else {
            if sibling_left != EMPTY_REF {
                self.mut_node(sibling_left).color = Color::Black;
            }
            self.rotate_right(p_index)
        }
    }

    fn handle_red_sibling(&mut self, n_index: u32, s_index: u32) {
        // Recolor...

        self.mut_node(s_index).color = Color::Black;
        let p_index = self.node(n_index).parent;
        let parent = self.mut_node(p_index);

        parent.color = Color::Red;

        // ... and rotate
        if n_index == parent.left {
            self.rotate_left(p_index)
        } else {
            self.rotate_right(p_index)
        }
    }

    fn get_sibling(&self, n_index: u32) -> u32 {
        let p_index = self.node(n_index).parent;
        let parent = self.node(p_index);
        assert!(n_index == parent.left || n_index == parent.right);
        if n_index == parent.left {
            parent.right
        } else {
            parent.left
        }
    }

    pub fn find_left_minimum(&self, n_index: u32) -> u32 {
        let mut i = n_index;
        while self.node(i).left != EMPTY_REF {
            i = self.node(i).left
        }
        i
    }

    pub fn find(&self, value: T) -> Option<T> {
        let mut index = self.root;

        while index != EMPTY_REF {
            let node = self.node(index);
            if node.value == value {
                return Some(node.value.clone());
            } else if value < node.value {
                index = node.left;
            } else {
                index = node.right;
            }
        }

        None
    }

    pub fn find_index(&self, value: T) -> u32 {
        let mut index = self.root;

        while index != EMPTY_REF {
            let node = self.node(index);
            if node.value == value {
                return index;
            } else if value < node.value {
                index = node.left;
            } else {
                index = node.right;
            }
        }

        EMPTY_REF
    }

    pub fn height(&self) -> usize {
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
}