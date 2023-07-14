#[cfg(test)]
mod tests {
    use super::super::*;

    #[test]
    fn test_binary_search() {
        let mut vec = vec![-10, 16, -115, 18, 29, -16, 37, 92, -1001, 9];
        vec.sort_unstable();
        for i in 0..vec.len() {
            let ind = binary_search(vec[i], &vec);
            assert_eq!(ind.unwrap(), i);
        }
        let fail_find = binary_search(vec[0] - 1, &vec);
        assert!(fail_find.is_err());
        if let Err(index) = fail_find {
            assert_eq!(index, 0);
        }
    }
}
