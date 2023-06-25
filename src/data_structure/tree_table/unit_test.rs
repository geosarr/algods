#[cfg(test)]
mod tests {
    use super::super::{BSearchTree, BTreeTable, OrdVecTable};

    #[test]
    fn test_ordered_vec_symbol_table() {
        assert!(OrdVecTable::<usize, isize>::new().is_empty());
        let mut st = OrdVecTable::<usize, &str>::init(2, "test2");
        assert_eq!(st.len(), 1);
        assert!(st.contains(&2));
        st.insert(0, "test0");
        assert_eq!(st.get(&0), Some(&"test0"));
        assert_eq!(st.get(&2), Some(&"test2"));
        // println!("{:?}", st);
        st.delete(&0);
        assert!(!st.contains(&0));
        st.insert(1, "test1");
        assert_eq!(st.min(), Some(&0));
        assert_eq!(st.max(), Some(&2));
        assert_eq!(st.floor(&1), Some(&1));
        assert_eq!(st.floor(&3), Some(&2));
        assert_eq!(st.ceil(&4), None);
        assert_eq!(st.floor(&4), Some(&2));

        let a = st.get(&2).unwrap();
        println!("{a}");
        st.delete(&2);
    }

    #[test]
    fn test_binary_search_tree() {
        let mut st = BSearchTree::<usize, &str>::init(10, "test10");
        assert_eq!(st.len(), 1);
        st.insert(1, "test1");
        st.insert(5, "test5");
        assert_eq!(st.get(&10), Some(&"test10"));
        assert_eq!(st.get(&0), None);
        assert_eq!(st.get(&1), Some(&"test1"));
        assert!(st.contains(&5));
        println!("{:#?}", st);
        assert_eq!(st.floor(&5), Some(&5));
        assert_eq!(st.floor(&1), Some(&1));
        assert_eq!(st.floor(&4), Some(&1));
        assert_eq!(st.floor(&0), None);
        assert_eq!(st.floor(&15), Some(&10));
        assert_eq!(st.min(), Some(&1));
        assert_eq!(st.max(), Some(&10));
        st.insert(20, "test20");
        assert_eq!(st.max(), Some(&20));
    }

    #[test]
    fn test_b_tree_symbol_table() {
        assert!(BTreeTable::<usize, String>::new().is_empty());
        let mut st = BTreeTable::<usize, &str>::init(5, "test5");
        assert_eq!(st.len(), 1);
        assert!(!st.is_empty());
        assert_eq!(st.get(&5), Some(&"test5"));
        assert_eq!(st.get(&0), None);
        assert_eq!(st.strict_floor(&10), Some(&5));
        assert_eq!(st.ceil(&2), Some(&5));
        st.insert(2, "test2");
        assert_eq!(st.ceil(&2), Some(&2));
        st.insert(3, "test3");
        st.insert(10, "test10");
        assert_eq!(st.range_search(&0, &5), vec![&2, &3]);
    }
}
