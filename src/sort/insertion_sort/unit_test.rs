#[cfg(test)]
mod tests {
    use super::super::InsertionSort;

    #[test]
    fn test_insertion_sort() {
        let mut v = vec![-10, 16, -115, 18, 29, -16, 37, 92, -1001, 9];
        let insert = InsertionSort { vec: v.clone() };
        let vec = insert.into_sorted_vec();
        v.sort(); // std sort of a vec
        assert_eq!(vec, v);
    }
}
