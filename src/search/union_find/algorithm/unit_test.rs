#[cfg(test)]
mod tests {
    use super::super::*;

    #[test]
    fn test_algo_format() {
        let algo = UnionFindAlgorithm::QuickFind;
        assert_eq!(algo.to_string(), "QuickFind");
        let algo = UnionFindAlgorithm::QuickUnion;
        assert_eq!(algo.to_string(), "QuickUnion");
        let algo = UnionFindAlgorithm::WeightedQuickUnion;
        assert_eq!(algo.to_string(), "WeightedQuickUnion");
        let algo = UnionFindAlgorithm::WeightedQuickUnionPathComp;
        assert_eq!(algo.to_string(), "WeightedQuickUnionPathComp");
    }

    #[test]
    fn test_parse_str_to_algo() {
        let bad_algo = UnionFindAlgorithm::from_str("NotImplementedAlgo");
        let error = Err(ParseUnionFindAlgorithmError);
        assert_eq!(bad_algo, error);
    }
}
