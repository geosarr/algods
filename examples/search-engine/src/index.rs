use crate::collection::{Collection, Document};
use crate::preprocessing::{character_ngram, is_not_punct, preprocess};
use std::collections::{HashMap, HashSet};
pub struct InvertedIndex {
    index: HashMap<String, Vec<usize>>, // stores the postings
    raw_freq: HashMap<usize, HashMap<String, usize>>, // stores the number of occurrences of tokens in the documents they appear
    char_t_index: HashMap<String, HashSet<String>>,   // character to term index
    t_char_index: HashMap<String, HashSet<String>>,   // term to character index
    include_char_index: bool, // says whether or not to include the (term to) character (to term) index
    ngram: usize,             // the number of characters to consider for the character n-gram index
}

impl InvertedIndex {
    pub fn new() -> Self {
        Self {
            index: HashMap::new(),
            raw_freq: HashMap::new(),
            char_t_index: HashMap::new(),
            t_char_index: HashMap::new(),
            include_char_index: true,
            ngram: 2,
        }
    }

    pub fn from(include_char_index: bool, ngram: usize) -> Self {
        let mut inv_index = Self::new();
        inv_index.include_char_index = include_char_index;
        inv_index.ngram = ngram;
        inv_index
    }

    pub fn index(&self) -> &HashMap<String, Vec<usize>> {
        &self.index
    }

    pub fn raw_freq(&self) -> &HashMap<usize, HashMap<String, usize>> {
        &self.raw_freq
    }

    pub fn posting(&self, tok: &str) -> Option<&[usize]> {
        self.index.get(tok).map(|posting| posting.as_slice())
    }

    pub fn index_document(&mut self, document: Document, collection: &mut Collection) {
        let doc_id = document.id();
        if !collection.contains(&doc_id) {
            collection.insert(doc_id, document);
        }

        let terms = preprocess(collection.document(&doc_id));
        // Character indexing the document
        if self.include_char_index {
            for term in terms.keys() {
                let chars = character_ngram(term, self.ngram); // String, usize -> HashSet
                for _char in &chars {
                    if let Some(h) = self.char_t_index.get_mut(_char) {
                        (*h).insert(term.to_string());
                    } else {
                        self.char_t_index.insert(_char.to_string(), HashSet::new());
                    }
                }
                self.t_char_index.insert(term.to_string(), chars);
            }
        }
        // Invert indexing the document
        for token in terms.keys() {
            if let Some(posting) = self.index.get_mut(token) {
                (*posting).push(doc_id); // works if the documents are indexed iteratively with increasing IDs.
            } else {
                self.index.insert(token.to_string(), vec![doc_id]);
            }
        }
        self.raw_freq.insert(doc_id, terms);
    }
}

#[derive(Debug, Clone)]
pub struct PositionalPosting {
    pub docs: Vec<PositionInDocument>,
}
impl PositionalPosting {
    pub fn with_capacity(size: usize) -> Self {
        Self {
            docs: Vec::with_capacity(size),
        }
    }
    pub fn push(&mut self, value: PositionInDocument) {
        self.docs.push(value)
    }
}

#[derive(Debug, Clone)]
pub struct PositionInDocument {
    pub id: usize,
    pub positions: TokenPosition,
}

#[derive(Debug, Clone)]
pub struct TokenPosition {
    pub pos: Vec<usize>,
}
impl TokenPosition {
    pub fn push(&mut self, position: usize) {
        self.pos.push(position);
    }
}

pub struct PositionalIndex {
    index: HashMap<String, PositionalPosting>, // stores the postings
    char_t_index: HashMap<String, HashSet<String>>, // character to term index
    t_char_index: HashMap<String, HashSet<String>>, // term to character index
    include_char_index: bool, // says whether or not to include the (term to) character (to term) index
    ngram: usize,             // the number of characters to consider for the character n-gram index
}

impl PositionalIndex {
    pub fn new() -> Self {
        Self {
            index: HashMap::new(),
            char_t_index: HashMap::new(),
            t_char_index: HashMap::new(),
            include_char_index: true,
            ngram: 2,
        }
    }
    pub fn index(&self) -> &HashMap<String, PositionalPosting> {
        &self.index
    }
    pub fn from(include_char_index: bool, ngram: usize) -> Self {
        let mut inv_index = Self::new();
        inv_index.include_char_index = include_char_index;
        inv_index.ngram = ngram;
        inv_index
    }

    pub fn index_document(&mut self, document: Document, collection: &mut Collection) {
        let doc_id = document.id();
        if !collection.contains(&doc_id) {
            collection.insert(doc_id, document);
        }
        let terms = preprocess(collection.document(&doc_id));
        // Character indexing the document
        if self.include_char_index {
            for term in terms.keys() {
                let chars = character_ngram(term, self.ngram); // String, usize -> HashSet
                for _char in &chars {
                    if let Some(h) = self.char_t_index.get_mut(_char) {
                        (*h).insert(term.to_string());
                    } else {
                        self.char_t_index.insert(_char.to_string(), HashSet::new());
                    }
                }
                self.t_char_index.insert(term.to_string(), chars);
            }
        }
        // Positional indexing
        let mut content = collection.document(&doc_id).content().to_lowercase();
        content.retain(is_not_punct);
        let terms: Vec<&str> = content.split_whitespace().collect();
        for (position, token) in terms.into_iter().enumerate() {
            if !self.index.contains_key(token) {
                // The first element is the document frequency (1 here) the following elements are the document ID followed
                // by the raw frequency of the term and its positions in the corresponding documents.
                let doc = PositionInDocument {
                    id: doc_id,
                    positions: TokenPosition {
                        pos: vec![position],
                    },
                };
                self.index
                    .insert(token.to_string(), PositionalPosting { docs: vec![doc] });
            } else {
                // The term appeared at least once (either in the current document or in a previous one)
                // Here we assume that the doc_id's are provided in an increasing order.
                let ids = self.index[token]
                    .docs
                    .iter()
                    .map(|doc| doc.id)
                    .collect::<Vec<_>>();
                if let Ok(pos) = ids.binary_search(&doc_id) {
                    // the term appears at least a second time in the current document
                    let doc_pos = &mut self.index.get_mut(token).unwrap().docs[pos];
                    assert_eq!(doc_pos.id, doc_id);
                    doc_pos.positions.push(position);
                } else {
                    // The term appears in a new document
                    // Here we assume again that the doc_id's are provided in an increasing order.
                    self.index
                        .get_mut(token)
                        .unwrap()
                        .docs
                        .push(PositionInDocument {
                            id: doc_id,
                            positions: TokenPosition {
                                pos: vec![position],
                            },
                        });
                }
            }
        }
    }
}
