#[cfg(test)]
mod tests {
    use rand::prelude::SliceRandom;
    use rand::rng;
    use i_tree::ord::list::SortedList;
    use i_tree::ord::sort::SortedCollection;
    use i_tree::ord::tree::BinTree;

    #[test]
    fn test_random_00() {
        let n = 3;
        let template: Vec<i32> = (1..n).collect();
        let mut rng = rng();
        for _ in 0..1000 {
            let mut array: Vec<i32> = template.clone();
            array.shuffle(&mut rng);
            let mut tree = BinTree::new(300);
            let mut list = SortedList::new(array.len());

            while let Some(val) = array.pop() {
                tree.insert(val, val);
                list.insert(val, val);

                for i in 0..n {
                    let a = tree.get_value(i);
                    let b = list.get_value(i);
                    
                    assert_eq!(a, b);
                }
            }
        }
    }

    #[test]
    fn test_random_01() {
        let n = 100;
        let template: Vec<i32> = (1..n).collect();
        let mut rng = rng();
        for _ in 0..1000 {
            let mut array: Vec<i32> = template.clone();
            array.shuffle(&mut rng);
            let mut tree = BinTree::new(300);
            let mut list = SortedList::new(array.len());

            while let Some(val) = array.pop() {
                tree.insert(val, val);
                list.insert(val, val);

                for i in 0..n {
                    let a = tree.get_value(i);
                    let b = list.get_value(i);
                    assert_eq!(a, b);
                }
            }
        }
    }

    #[test]
    fn test_random_02() {
        let n = 100;
        let template: Vec<i32> = (1..n).collect();
        let mut rng = rng();
        let mut values = Vec::new();
        for i in 0..1000 {
            values.clear();
            let mut array: Vec<i32> = template.clone();
            array.shuffle(&mut rng);
            let mut tree = BinTree::new(array.len());
            let mut list = SortedList::new(array.len());

            while let Some(val) = array.pop() {
                tree.insert(val, val);
                list.insert(val, val);
                values.push(val);
                for i in 0..n {
                    let a = tree.get_value(i);
                    let b = list.get_value(i);
                    assert_eq!(a, b);
                }
            }

            if i % 5 == 0 && !values.is_empty() {
                let val = values.pop().unwrap();
                tree.delete(val);
                list.delete(val);
            }

        }
    }
}