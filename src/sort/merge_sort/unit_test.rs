#[cfg(test)]
mod tests {
    use super::super::{MergeSort, MergeSortAlgorithm};

    #[test]
    fn test_merge_sort() {
        let mut v = vec![-10, 16, -115, 18, 29, -16, 37, 92, -1001, 9];
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
