#[cfg(test)]
mod tests {
    use crate::preprocessing::*;
    use std::collections::{HashMap, HashSet};

    #[test]
    fn test_character_ngram() {
        let words = "rustyboy".to_owned();
        let char_gram = character_ngram(&words, 4);
        let expected_result = HashSet::from(["rust", "usty", "styb", "tybo", "yboy"]);
        let expected_result = expected_result
            .iter()
            .map(|s| s.to_string())
            .collect::<HashSet<_>>();
        assert_eq!(char_gram, expected_result);

        let char_gram = character_ngram(&words, 3);
        let expected_result = HashSet::from(["rus", "ust", "sty", "tyb", "ybo", "boy"]);
        let expected_result = expected_result
            .iter()
            .map(|s| s.to_string())
            .collect::<HashSet<_>>();
        assert_eq!(char_gram, expected_result);
    }

    #[test]
    fn test_counter() {
        let vec = vec!["a", "b", "b", "c", "d", "a", "a"];
        let mut counter = Counter::new();
        let a = "a".to_string();
        let b = "b".to_string();
        let c = "c".to_string();
        let d = "d".to_string();
        let expected_result = HashMap::from([(a, 3), (b, 2), (c, 1), (d, 1)]);
        assert_eq!(counter.count(vec), expected_result);
    }
}
