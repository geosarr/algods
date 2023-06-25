// mod indice;
// #[cfg(test)]
// mod unit_test;
use crate::collection::{Collection, Document};
use crate::indice::InvertedIndex;
use flate2::read::MultiGzDecoder;
use pbr::ProgressBar;
use quick_xml::events::Event;
use quick_xml::Reader;
use std::fs::File;
use std::io::BufReader;

pub struct Loader {
    path: String,
}

impl Loader {
    pub fn new() -> Self {
        Self {
            path: String::new(),
        }
    }
    pub fn init(path: String) -> Self {
        Self { path }
    }
    pub fn load(&self, max_num_doc: usize) -> (InvertedIndex, Collection) {
        let file = File::open(self.path.as_str()).unwrap();
        let mut index = InvertedIndex::new();
        let mut collection = Collection::new();
        let mut flag_abs = false;
        let mut doc_id = 0;
        let bufreader = BufReader::new(file);
        let mgz = MultiGzDecoder::new(bufreader);
        let mut reader = Reader::from_reader(BufReader::new(mgz));
        let mut buf = Vec::new();
        let mut pb = ProgressBar::new(max_num_doc as u64);
        pb.format("╢▌▌░╟");
        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(ref e)) => {
                    if let b"abstract" = e.name().as_ref() {
                        flag_abs = true;
                    }
                }
                Ok(Event::End(ref e)) => {
                    if let b"abstract" = e.name().as_ref() {
                        flag_abs = false;
                    }
                }
                Ok(Event::Text(e)) => {
                    let text = e.unescape().unwrap().into_owned();
                    if text.len() >= 10 && flag_abs {
                        let doc = Document::init(doc_id + 1, text);
                        index.index_document(doc, &mut collection);
                        flag_abs = false;
                        doc_id += 1;
                        pb.inc();
                    }
                }
                Ok(Event::Eof) => break,
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (),
            }
            if collection.len() == max_num_doc {
                break;
            }
        }
        buf.clear();
        // println!("{:#?}", collection.get_document(&5));
        // println!("Total abstracts {}", collection.len());
        // println!("{:#?}", index.index());
        // println!("{:#?}", index.raw_freq());
        // println!("{:#?}", collection.get_document(&9));
        // println!("{:#?}", collection.get_document(&8));
        // println!("{:#?}", collection.get_document(&5));
        // println!("{:#?}", collection.get_document(&4));
        return (index, collection);
    }
}
