use std::collections::HashSet;

use crate::{
    collection::{Collection, Document},
    index::{PositionInDocument, PositionalIndex, PositionalPosting, TokenPosition},
    preprocessing::simple_preprocess,
};

pub struct Phrase {
    k: usize,
}
impl Phrase {
    pub fn new() -> Self {
        Self { k: 5 }
    }
    pub fn from(k: usize) -> Self {
        Self { k }
    }
    pub fn retrieve<'a>(
        &self,
        query: &str,
        index: &PositionalIndex,
        collection: &'a Collection,
    ) -> Vec<&'a Document> {
        let processed_query = simple_preprocess(query);
        let processed_query: HashSet<_> = processed_query.split_whitespace().collect();
        let postings = processed_query
            .iter()
            .filter_map(|tok| index.posting(tok))
            .collect::<Vec<_>>();
        positional_intersect(&postings, self.k)
            .iter()
            .map(|id| collection.document(id))
            .collect()
    }
}
fn intersect_two(
    posting1: &PositionalPosting,
    posting2: &PositionalPosting,
    k: usize,
) -> PositionalPosting {
    let (mut p1, mut p2) = (0, 0);
    let (n1, n2) = (posting1.docs.len(), posting2.docs.len());
    let mut result = PositionalPosting::with_capacity(std::cmp::min(n1, n2) + 1);

    while (p1 < n1) && (p2 < n2) {
        let (doc_id1, doc_id2) = (posting1.docs[p1].id, posting2.docs[p2].id);
        if doc_id1 == doc_id2 {
            let mut l: Vec<usize> = vec![];
            // lists of terms positions in the documents
            let (l1, l2) = (
                &posting1.docs[p1].positions.pos,
                &posting2.docs[p2].positions.pos,
            );
            let mut pp1 = 0;
            let (nn1, nn2) = (l1.len(), l2.len());
            while pp1 < nn1 {
                let mut pp2 = 0;
                while pp2 < nn2 {
                    let dist = l2[pp2] - l1[pp1];
                    if (1 <= dist) && (dist <= k) {
                        // the term t2 is in k words ahead from the term t1 (after preprocessing the document)
                        l.push(l2[pp2]);
                    } else if l2[pp2] > l1[pp1] + k {
                        break;
                    }
                    pp2 += 1;
                }
                pp1 += 1;
            }
            if l.len() > 0 {
                // To make sure that only documents that match the phrase "t1 t2" (after preprocessing) are taken into account
                result.push(PositionInDocument {
                    id: doc_id1,
                    positions: TokenPosition { pos: l },
                });
            }
            p1 += 1;
            p2 += 1;
        } else if doc_id1 < doc_id2 {
            p1 += 1
        } else {
            p2 += 1
        }
    }
    result
}

fn positional_intersect(postings: &[&PositionalPosting], k: usize) -> Vec<usize> {
    let mut result = postings[0].clone();
    let mut rest = &postings[1..];
    while !rest.is_empty() && !result.docs.is_empty() {
        result = intersect_two(&result, &rest[0], k);
        rest = &rest[1..]
    }
    result.docs.iter().map(|posting| posting.id).collect()
}
