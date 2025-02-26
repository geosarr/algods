use crate::collection::{Collection, Document};
use crate::index::InvertedIndex;
use crate::preprocessing::simple_preprocess;
use std::cmp::min;
use std::collections::HashSet;

pub struct Boolean {
    query_type: BooleanQuery,
}
pub enum BooleanQuery {
    And,
    Or,
}

impl Boolean {
    pub fn new() -> Self {
        Self {
            query_type: BooleanQuery::Or,
        }
    }
    pub fn retrieve<'a>(
        &self,
        query: &str,
        index: &InvertedIndex,
        collection: &'a Collection,
    ) -> Vec<&'a Document> {
        let processed_query = simple_preprocess(query);
        let processed_query: HashSet<_> = processed_query.split_whitespace().collect();
        let postings = processed_query
            .iter()
            .filter_map(|tok| index.posting(tok))
            .collect();
        let matching_postings = match self.query_type {
            BooleanQuery::And => op_many(postings, intersect),
            BooleanQuery::Or => op_many(postings, union),
        };
        matching_postings
            .iter()
            .map(|id| collection.document(id))
            .collect()
    }
}

fn op_many<O>(list_posts: Vec<&[usize]>, operation: O) -> Vec<usize>
where
    O: Fn(&[usize], &[usize]) -> Vec<usize>,
{
    if list_posts.is_empty() {
        return vec![];
    }
    // TODO: add sorting posting by incresing freq for intersection operation
    let mut rest = &list_posts[1..];
    let mut result = list_posts[0];
    let mut _temp = Vec::new();
    while !rest.is_empty() && !result.is_empty() {
        let posting = rest[0];
        _temp = operation(result, posting);
        result = &_temp;
        rest = &rest[1..];
    }
    return result.to_vec();
}

fn union(post1: &[usize], post2: &[usize]) -> Vec<usize> {
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

fn intersect(post1: &[usize], post2: &[usize]) -> Vec<usize> {
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
