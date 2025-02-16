use crate::collection::Document;
use crate::constant::PUNCTUATION;
use std::collections::{HashMap, HashSet};

fn slice_to_string(slice: &[char]) -> String {
    let mut s = String::with_capacity(slice.len());
    for ch in slice {
        s.push(*ch);
    }
    s
}
pub fn character_ngram(word: &str, size: usize) -> HashSet<String> {
    let n = word.chars().count();
    if n <= size {
        return HashSet::from([word.to_string()]);
    }
    let word = word.trim().chars().collect::<Vec<_>>();
    (0..n - size + 1)
        .map(|i| slice_to_string(&word[i..i + size]))
        .collect()
}

pub fn is_not_punct(character: char) -> bool {
    !PUNCTUATION.contains(character)
}

pub fn preprocess(doc: &Document) -> HashMap<String, usize> {
    let mut content = doc.content().to_lowercase();
    content.retain(is_not_punct);
    let content = content.split_whitespace().collect();
    let mut counter = Counter::new();
    counter.count(content)
}

pub struct Counter {}

impl Counter {
    pub fn new() -> Self {
        Self {}
    }
    pub fn count(&mut self, content: Vec<&str>) -> HashMap<String, usize> {
        let mut counter = HashMap::with_capacity(content.len());
        for word in content {
            if let Some(count) = counter.get_mut(word) {
                *count += 1;
            } else {
                counter.insert(word.to_string(), 1);
            }
        }
        counter
    }
}
