use std::collections::HashMap;

use rocket::{get, launch, routes, State};
use search::{
    index::{InvertedIndex, PositionalIndex},
    loader::Loader,
    model::{Boolean, Phrase},
    SearchEngine,
};
mod engine;

pub enum ModelType {
    Boolean(Boolean),
    Phrase(Phrase),
}
impl ModelType {
    pub fn get_boolean(&self) -> &Boolean {
        if let Self::Boolean(model) = self {
            model
        } else {
            panic!("Not boolean model")
        }
    }
    pub fn get_phrase(&self) -> &Phrase {
        if let Self::Phrase(model) = self {
            model
        } else {
            panic!("Not phrase model")
        }
    }
}

pub enum IndexType {
    Inverted(InvertedIndex),
    Positional(PositionalIndex),
}
impl IndexType {
    pub fn get_inverted(&self) -> &InvertedIndex {
        if let Self::Inverted(index) = self {
            index
        } else {
            panic!("Not inverted index")
        }
    }
    pub fn get_positional(&self) -> &PositionalIndex {
        if let Self::Positional(index) = self {
            index
        } else {
            panic!("Not positional index")
        }
    }
}

pub type Engine = SearchEngine<HashMap<&'static str, IndexType>, HashMap<&'static str, ModelType>>;

#[get("/boolean/search/<query>")]
async fn boolean_search(state: &State<Engine>, query: String) -> String {
    state.models["boolean"]
        .get_boolean()
        .retrieve(
            &query,
            state.indices["inverted"].get_inverted(),
            &state.collection,
        )
        .iter()
        .map(|doc| format!("{}\n\n", doc.content()))
        .take(100)
        .collect()
}

#[get("/phrase/search/<query>")]
async fn phrase_search(state: &State<Engine>, query: String) -> String {
    state.models["phrase"]
        .get_phrase()
        .retrieve(
            &query,
            state.indices["positional"].get_positional(),
            &state.collection,
        )
        .iter()
        .map(|doc| format!("{}\n\n", doc.content()))
        .take(100)
        .collect()
}

#[launch]
async fn rocket() -> _ {
    let loader = Loader::from("enwiki-latest-abstract.xml.gz".to_string()).unwrap();
    let models = HashMap::from([
        ("boolean", ModelType::Boolean(Boolean::new())),
        ("phrase", ModelType::Phrase(Phrase::new())),
    ]);
    let (positional, inverted, collection) = loader.load(50000);
    let indices = HashMap::from([
        ("inverted", IndexType::Inverted(inverted)),
        ("positional", IndexType::Positional(positional)),
    ]);
    rocket::build()
        .mount("/", routes![boolean_search, phrase_search])
        .manage(SearchEngine {
            indices,
            collection,
            models,
        })
}
