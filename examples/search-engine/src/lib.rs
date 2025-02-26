// use collection::{Collection, Document};
// use index::InvertedIndex;
// use model::{Boolean, Phrase};

use collection::{Collection, Document};
use index::{InvertedIndex, PositionalIndex};
use model::{Boolean, Phrase};

pub mod collection;
pub mod constant;
pub mod index;
pub mod loader;
pub mod model;
pub mod preprocessing;
mod unit_test;

pub trait Model<I> {
    fn retrieve<'a>(&self, query: &str, index: &I, collection: &'a Collection)
        -> Vec<&'a Document>;
}

macro_rules! impl_model {
    ($model:ident, $ty_index:ty) => {
        impl Model<$ty_index> for $model {
            fn retrieve<'a>(
                &self,
                query: &str,
                index: &$ty_index,
                collection: &'a Collection,
            ) -> Vec<&'a Document> {
                $model::retrieve(&self, query, index, collection)
            }
        }
    };
}
impl_model!(Boolean, InvertedIndex);
impl_model!(Phrase, PositionalIndex);
pub trait Index {
    fn new() -> Self;
    fn index_document(&mut self, document: Document, collection: &mut Collection);
}
macro_rules! impl_index {
    ($name:ident) => {
        impl Index for $name {
            fn new() -> Self {
                Self::new()
            }
            fn index_document(&mut self, document: Document, collection: &mut Collection) {
                Self::index_document(self, document, collection)
            }
        }
    };
}
impl_index!(InvertedIndex);
impl_index!(PositionalIndex);
