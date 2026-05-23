#[cfg(test)]
mod tests {
    use i_tree::EMPTY_REF;
    use i_tree::set::list::SetList;
    use i_tree::set::sort::{KeyValue, SetCollection};
    use i_tree::set::tree::SetTree;

    #[derive(Clone, Default, Debug, PartialEq, Eq)]
    struct Item {
        key: i32,
        value: i32,
    }

    impl Item {
        fn new(key: i32) -> Self {
            Self { key, value: key }
        }
    }

    impl KeyValue<i32> for Item {
        fn key(&self) -> &i32 {
            &self.key
        }
    }

    fn run_used_set_operations<S: SetCollection<i32, Item>>(store: &mut S) {
        store.insert(Item::new(1));
        store.insert(Item::new(4));
        store.insert(Item::new(8));

        let i0 = store.first_index_less_by(|key| key.cmp(&0));
        assert_eq!(i0, EMPTY_REF);

        let i1 = store.first_index_less_by(|key| key.cmp(&5));
        assert_eq!(unsafe { store.value_by_index(i1) }.key, 4);

        unsafe { store.value_by_index_mut(i1) }.value = 40;
        assert_eq!(unsafe { store.value_by_index(i1) }.value, 40);

        let i2 = store.first_index_less_by(|key| key.cmp(&9));
        let i3 = store.index_before(i2);
        assert_eq!(unsafe { store.value_by_index(i3) }.key, 4);

        store.delete_by_index(i3);
        let i4 = store.first_index_less_by(|key| key.cmp(&5));
        assert_eq!(unsafe { store.value_by_index(i4) }.key, 1);
    }

    #[test]
    fn list_supports_used_set_operations() {
        let mut list = SetList::new(8);
        run_used_set_operations(&mut list);
    }

    #[test]
    fn tree_supports_used_set_operations() {
        let mut tree = SetTree::new(8);
        run_used_set_operations(&mut tree);
    }
}
