#[cfg(test)]
mod tests {
    use super::super::{MergeSort, MergeSortAlgorithm};
    use crate::utils::{gen_vec_rand_int, RandKind};

    #[test]
    fn test_merge_sort() {
        let mut v = gen_vec_rand_int(10000, RandKind::Full);
        let mbup = MergeSort {
            vec: v.clone(),
            algo: MergeSortAlgorithm::BottomUp,
        };
        let mrec = MergeSort {
            vec: v.clone(),
            algo: MergeSortAlgorithm::Recursive,
        };
        let vec1 = mbup.into_sorted_vec();
        let vec2 = mrec.into_sorted_vec();
        v.sort(); // std sort of a vec
        assert_eq!(vec1, v);
        assert_eq!(vec2, v);
    }
}
