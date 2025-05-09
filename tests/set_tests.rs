#[cfg(test)]
mod tests {
    use rand::prelude::SliceRandom;
    use rand::rng;
    use i_tree::set::list::SetList;
    use i_tree::set::sort::SetCollection;
    use i_tree::set::tree::SetTree;

    #[test]
    fn test_00() {
        let mut tree = SetTree::new(2);
        tree.insert(1);
        tree.insert(2);
        let a1 = tree.get_value(&2);
        assert_eq!(*a1.unwrap(), 2);
    }

    #[test]
    fn test_random_00() {
        let n = 20;
        let template: Vec<i32> = (1..n).collect();
        let mut rng = rng();
        for _ in 0..100 {
            let mut array: Vec<i32> = template.clone();
            array.shuffle(&mut rng);
            let mut tree = SetTree::new(300);
            let mut list = SetList::new(array.len());

            while let Some(val) = array.pop() {
                tree.insert(val);
                list.insert(val);

                for i in 0..n {
                    let a = tree.get_value(&i);
                    let b = list.get_value(&i);

                    assert_eq!(a, b);
                }
            }
        }
    }

    #[test]
    fn test_random_01() {
        let n = 60;
        let template: Vec<i32> = (1..n).collect();
        let mut rng = rng();
        for _ in 0..1000 {
            let mut array: Vec<i32> = template.clone();
            array.shuffle(&mut rng);
            let mut tree = SetTree::new(300);
            let mut list = SetList::new(array.len());

            while let Some(val) = array.pop() {
                tree.insert(val);
                list.insert(val);

                for i in 0..n {
                    let a = tree.get_value(&i);
                    let b = list.get_value(&i);
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
        for _ in 0..1000 {
            values.clear();
            let mut array: Vec<i32> = template.clone();
            array.shuffle(&mut rng);
            let mut tree = SetTree::new(array.len());
            let mut list = SetList::new(array.len());

            while let Some(val) = array.pop() {
                tree.insert(val);
                list.insert(val);
                values.push(val);
                for i in 0..n {
                    let a = tree.get_value(&i);
                    let b = list.get_value(&i);
                    assert_eq!(a, b);
                }

                if values.len() > 16 {
                    let val = values.pop().unwrap();
                    tree.delete(&val);
                    list.delete(&val);
                }
            }
        }
    }
}