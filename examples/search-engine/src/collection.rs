use std::collections::HashMap;
#[derive(Debug)]
pub struct Collection {
    data: HashMap<usize, Document>,
}
impl Collection {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }
    pub fn len(&self) -> usize {
        self.data.len()
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    pub fn insert(&mut self, id: usize, doc: Document) {
        self.data.insert(id, doc);
    }
    pub fn get_document(&self, id: &usize) -> &Document {
        &self.data[id]
    }
    pub fn contains(&self, id: &usize) -> bool {
        self.data.contains_key(id)
    }
}

#[derive(Debug, Clone)]
pub struct Document {
    id: usize,
    content: String,
}

impl Document {
    pub fn init(doc_id: usize, content: String) -> Self {
        Self {
            id: doc_id,
            content,
        }
    }
    pub fn id(&self) -> usize {
        self.id
    }
    pub fn content(&self) -> &String {
        &self.content
    }
}
