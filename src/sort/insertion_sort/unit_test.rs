#[cfg(test)]
mod tests {
    use super::super::InsertionSort;
    use crate::utils::{gen_vec_rand_int, RandKind};

    #[test]
    fn test_insertion_sort() {
        let mut v = gen_vec_rand_int(1000, RandKind::Full);
        let insert = InsertionSort { vec: v.clone() };
        let vec = insert.into_sorted_vec();
        v.sort(); // std sort of a vec
        assert_eq!(vec, v);
    }
}
