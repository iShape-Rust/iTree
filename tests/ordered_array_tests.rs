#[cfg(test)]
mod tests {
    use rand::thread_rng;
    use rand::seq::SliceRandom;
    use i_tree::array::Array;
    use i_tree::tree::Tree;

    #[test]
    fn test_00() {
        let mut vec = vec![6, 3, 4, 8, 1];
        let mut tree = Tree::new(0, vec.len());
        for a in vec.iter() {
            tree.insert(a.clone());
        }

        let ordered = tree.ordered_list();
        vec.sort();

        assert_eq!(vec, ordered);
    }

    #[test]
    fn test_01() {
        let n = 100;
        for _ in 0..10000 {
            let mut vec = generate_and_shuffle_vector(n);
            let mut tree = Tree::new(0, n);
            for a in vec.iter() {
                tree.insert(a.clone());
            }

            let ordered = tree.ordered_list();
            vec.sort();

            assert_eq!(vec, ordered);
        }
    }

    fn generate_and_shuffle_vector(n: usize) -> Vec<i32> {
        let mut vec: Vec<i32> = (0..n as i32).collect();
        let mut rng = thread_rng();
        vec.shuffle(&mut rng);
        vec
    }

}