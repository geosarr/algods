#[cfg(test)]
mod tests {
    use super::super::*;

    #[test]
    fn test_percolation() {
        let mut p = Percolation::with_capacity(3, UnionFindAlgorithm::QuickFind);
        for i in 0..3 {
            for j in 0..3 {
                assert!(!p.is_open(i, j));
            }
        }
        p.open(0, 0);
        assert!(p.is_open(0, 0));
        p.open(1, 1);
        assert!(p.is_open(1, 1));
        p.open(1, 0);
        assert!(p.is_open(1, 0));
        assert!(p.is_full(1, 1));
        assert!(p.is_full(1, 0));
        // (0,0) corresponds to 0, (1,0) to 3, (1,1) to 4
        assert!(p.uf.connected(0, 3));
        assert!(p.uf.connected(0, 4));
        assert!(p.uf.connected(3, 4));
        p.open(2, 1);
        assert!(p.is_open(2, 1));
        assert!(p.uf.connected(4, 7));
        assert!(p.is_percolated());

        assert_eq!(p.threshold(), 4.0 as f32 / 9.0 as f32)
    }
}
