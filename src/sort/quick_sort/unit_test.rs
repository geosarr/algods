#[cfg(test)]
mod tests {
    use super::super::QuickSort;

    #[test]
    fn test_quick_sort() {
        let mut v: Vec<i16> = vec![-10, 16, -115, 18, 29, -16, 37, 92, -1001, 9];
        let quick = QuickSort { vec: v.clone() };
        let vec = quick.into_sorted_vec();
        v.sort(); // std sort of a vec
        assert_eq!(vec, v);
    }

    #[test]
    fn test_quick_select() {
        let mut v: Vec<i16> = vec![-1, 1, -80, 18, -32, -74, 37, 45, 16, 90];
        let mut quick = QuickSort { vec: v.clone() };
        let med = quick.select(v.len() / 2);
        v.sort(); // std sort of a vec
        assert_eq!(med, v[v.len() / 2]);
    }
}
