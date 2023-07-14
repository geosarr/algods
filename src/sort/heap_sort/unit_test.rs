#[cfg(test)]
mod tests {
    use super::super::{BinaryHeapSort, HeapSort};

    #[test]
    fn test_heap_sort() {
        let mut vec = vec![-10, 16, -115, 18, 29, -16, 37, 92, -1001, 9];
        let h = HeapSort::init(vec.clone());
        let v = h.into_sorted_vec();
        vec.sort();
        assert_eq!(vec, v);
    }

    #[test]
    fn test_binary_heap_sort() {
        let mut vec = vec![-10, 16, -115, 18, 29, -16, 37, 92, -1001, 9];
        let h = BinaryHeapSort::init(vec.clone());
        let v = h.into_sorted_vec();
        vec.sort();
        assert_eq!(vec, v);
    }
}
