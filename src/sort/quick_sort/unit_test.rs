#[cfg(test)]
mod tests {
    use super::super::QuickSort;
    use crate::utils::{gen_vec_rand_int, RandKind};

    #[test]
    fn test_quick_sort() {
        let mut v = gen_vec_rand_int(10000, RandKind::Full);
        let quick = QuickSort { vec: v.clone() };
        let vec = quick.into_sorted_vec();
        v.sort(); // std sort of a vec
        assert_eq!(vec, v);
    }

    #[test]
    fn test_quick_select() {
        let mut v = gen_vec_rand_int(1000, RandKind::Full);
        let mut quick = QuickSort { vec: v.clone() };
        let med = quick.select(500);
        v.sort(); // std sort of a vec
                  // assert!(med == v[499] || med == v[500]);
        assert_eq!(med, v[500]);
    }
}
