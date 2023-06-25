#[cfg(test)]
mod tests {
    use super::super::{BinaryHeapSort, HeapSort};
    use crate::utils::{gen_vec_rand_int, RandKind};

    #[test]
    fn test_heap_sort() {
        let mut vec = gen_vec_rand_int(100000, RandKind::Full);
        let h = HeapSort::init(vec.clone());
        let v = h.into_sorted_vec();
        vec.sort();
        assert_eq!(vec, v);
    }

    #[test]
    fn test_binary_heap_sort() {
        let mut vec = gen_vec_rand_int(100000, RandKind::Full);
        let h = BinaryHeapSort::init(vec.clone());
        let v = h.into_sorted_vec();
        vec.sort();
        assert_eq!(vec, v);
    }
}
