#[cfg(test)]
mod tests {
    use i_tree::ExpiredKey;
    use i_tree::key::exp::KeyExpCollection;
    use i_tree::key::list::KeyExpList;
    use i_tree::key::tree::KeyExpTree;
    use std::cmp::Ordering;

    #[derive(Debug, Clone, Copy)]
    struct Key {
        key: i32,
        exp: i32,
    }

    impl Key {
        fn new(key: i32, exp: i32) -> Self {
            Self { key, exp }
        }
    }

    impl Ord for Key {
        fn cmp(&self, other: &Self) -> Ordering {
            self.key.cmp(&other.key)
        }
    }

    impl Eq for Key {}

    impl PartialEq<Self> for Key {
        fn eq(&self, other: &Self) -> bool {
            self.key.eq(&other.key)
        }
    }

    impl PartialOrd<Self> for Key {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    impl ExpiredKey<i32> for Key {
        fn expiration(&self) -> i32 {
            self.exp
        }
    }

    fn run_queries<S: KeyExpCollection<Key, i32, i32>>(scan: &mut S) {
        scan.insert(Key::new(0, 10), 0, 0);
        scan.insert(Key::new(3, 10), 3, 0);
        scan.insert(Key::new(5, 10), 5, 0);
        scan.insert(Key::new(8, 10), 8, 0);

        assert_eq!(scan.first_less(0, -1, Key::new(4, 10)), 3);
        assert_eq!(scan.first_less(0, -1, Key::new(0, 10)), -1);

        assert_eq!(scan.first_less_or_equal_by(0, -1, |key| key.key.cmp(&5)), 5);
        assert_eq!(scan.first_less_or_equal_by(0, -1, |key| key.key.cmp(&7)), 5);

        scan.insert(Key::new(4, 1), 4, 0);
        assert_eq!(scan.first_less_or_equal_by(2, -1, |key| key.key.cmp(&5)), 5);

        scan.clear();
        assert_eq!(scan.first_less(0, -1, Key::new(9, 10)), -1);
    }

    #[test]
    fn list_collection_supports_used_key_queries() {
        let mut list = KeyExpList::new(8);
        run_queries(&mut list);
    }

    #[test]
    fn tree_collection_supports_used_key_queries() {
        let mut tree = KeyExpTree::new(8);
        run_queries(&mut tree);
    }
}
