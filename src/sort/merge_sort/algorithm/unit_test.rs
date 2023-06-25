#[cfg(test)]
mod tests {
    use super::super::{FromStr, MergeSortAlgorithm, ParseMergeSortAlgorithmError};

    #[test]
    fn test_algo_format() {
        let algo = MergeSortAlgorithm::Recursive;
        assert_eq!(algo.to_string(), "Recursive");
        let algo = MergeSortAlgorithm::BottomUp;
        assert_eq!(algo.to_string(), "BottomUp");
    }

    #[test]
    fn test_parse_str_to_algo() {
        let bad_algo = MergeSortAlgorithm::from_str("NotImplementedAlgo");
        let error = Err(ParseMergeSortAlgorithmError);
        assert_eq!(bad_algo, error);
    }
}
