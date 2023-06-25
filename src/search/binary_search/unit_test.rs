#[cfg(test)]
mod tests {
    use super::super::*;
    use crate::utils::{gen_vec_rand_int, RandKind};

    #[test]
    fn test_binary_search() {
        let mut vec = gen_vec_rand_int(1000000, RandKind::Full);
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
