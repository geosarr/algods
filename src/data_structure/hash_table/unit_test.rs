#[cfg(test)]
mod tests {
    use super::super::SepChainTable;

    #[test]
    fn test_separate_chaining_symbol_table() {
        let mut st = SepChainTable::<&str, isize>::init(31, "0", 0);
        assert_eq!(st.get(&"0"), Some(&0));
        assert!(!st.contains(&"1"));
        assert!(st.contains(&"0"));
        st.insert("10", 10);
        assert!(st.contains(&"10"));
        assert_eq!(st.get(&"10"), Some(&10));
        st.insert("10", 15);
        st.insert("0", 1);
        assert_eq!(st.get(&"10"), Some(&15));
        assert_eq!(st.get(&"0"), Some(&1));
        println!("{:?}", st);
    }
}
