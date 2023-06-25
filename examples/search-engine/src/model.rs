use crate::collection::{Collection, Document};
use crate::indice::InvertedIndex;
use crate::preprocessing::is_not_punct;
use std::cmp::min;
use std::collections::HashSet;
pub struct Boolean {
    query_type: BQueryType,
}
pub enum BQueryType {
    And,
    Or,
}

impl Boolean {
    pub fn new() -> Self {
        Self {
            query_type: BQueryType::Or,
        }
    }
    pub fn retrieve<'b, 'c>(
        &self,
        query: &'b str,
        index: &InvertedIndex,
        collection: &'c Collection,
    ) -> Vec<&'c Document> {
        let mut processed_query = query.to_lowercase();
        processed_query.retain(is_not_punct);
        let processed_query: HashSet<_> = processed_query.split_whitespace().collect();
        let collection_tokens: HashSet<_> = index.index().keys().map(|t| t.as_str()).collect(); // TODO: Add it to data preparation
        let overlap: HashSet<_> = processed_query.intersection(&collection_tokens).collect();
        let postings: Vec<_> = overlap.iter().map(|tok| index.get_posting(tok)).collect();
        if !postings.is_empty() {
            let matching_doc_id = match self.query_type {
                BQueryType::And => Self::intersect_many(postings),
                BQueryType::Or => Self::union_many(postings),
            };
            matching_doc_id
                .iter()
                .map(|id| collection.get_document(id))
                .collect()
        } else {
            vec![]
        }
    }
    fn union_many(list_posts: Vec<&Vec<usize>>) -> Vec<usize> {
        let mut rest = &list_posts[1..];
        let mut result = list_posts[0];
        let mut temp = Vec::new();
        while !rest.is_empty() && !result.is_empty() {
            let posting = rest[0];
            temp = Self::union(result, posting);
            result = &temp;
            rest = &rest[1..];
        }
        return result.to_vec();
    }
    fn intersect_many(list_posts: Vec<&Vec<usize>>) -> Vec<usize> {
        // TODO: add sorting posting by incresing freq
        let mut rest = &list_posts[1..];
        let mut result = list_posts[0];
        let mut temp = Vec::with_capacity(result.len());
        while !rest.is_empty() && !result.is_empty() {
            let posting = rest[0];
            temp = Self::intersect(result, posting);
            result = &temp;
            rest = &rest[1..];
        }
        return result.to_vec();
    }

    fn union(post1: &Vec<usize>, post2: &Vec<usize>) -> Vec<usize> {
        let mut p1 = 0;
        let mut p2 = 0;
        let n1 = post1.len();
        let n2 = post2.len();
        let mut result = Vec::with_capacity(n1 + n2);
        while p1 < n1 && p2 < n2 {
            if post1[p1] == post2[p2] {
                result.push(post1[p1]);
                p1 += 1;
                p2 += 1;
            } else if post1[p1] < post2[p2] {
                result.push(post1[p1]);
                p1 += 1;
            } else {
                result.push(post2[p2]);
                p2 += 1;
            }
        }
        while p1 < n1 {
            result.push(post1[p1]);
            p1 += 1;
        }
        while p2 < n2 {
            result.push(post2[p2]);
            p2 += 1;
        }
        return result;
    }

    fn intersect(post1: &Vec<usize>, post2: &Vec<usize>) -> Vec<usize> {
        let mut p1 = 0;
        let mut p2 = 0;
        let n1 = post1.len();
        let n2 = post2.len();
        let mut res = Vec::with_capacity(min(n1, n2));
        while p1 < n1 && p2 < n2 {
            if post1[p1] == post2[p2] {
                res.push(post1[p1]);
                p1 += 1;
                p2 += 1;
            } else if post1[p1] < post2[p2] {
                p1 += 1;
            } else {
                p2 += 1;
            }
        }
        return res;
    }
}
