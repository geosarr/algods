use crate::collection::{Collection, Document};
use crate::Index;
use flate2::read::MultiGzDecoder;
use pbr::ProgressBar;
use quick_xml::events::Event;
use quick_xml::Reader;
use std::fs::File;
use std::io::BufReader;

pub struct Loader {
    file: File,
}

impl Loader {
    pub fn from(path: String) -> std::io::Result<Self> {
        match File::open(path.as_str()) {
            Ok(file) => Ok(Self { file }),
            Err(error) => Err(error),
        }
    }
    pub fn load<I: Index>(&self, max_num_doc: usize) -> (I, Collection) {
        let mut index = I::new();
        let mut collection = Collection::new();
        let mut flag_abs = false;
        let mut doc_id = 0;
        let bufreader = BufReader::new(&self.file);
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
                        let doc = Document::from(doc_id + 1, text);
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
        // println!("{:#?}", index.index());
        // println!("{:#?}", collection.document(&5));
        // println!("{:#?}", collection.document(&1));
        return (index, collection);
    }
}

#[cfg(test)]
mod test {
    use crate::index::PositionalIndex;

    use super::*;

    #[test]
    fn test_loader() {
        match Loader::from("enwiki-latest-abstract.xml.gz".to_string()) {
            Ok(loader) => {
                loader.load::<PositionalIndex>(10);
            }
            Err(error) => println!("Error reading file, {error}"),
        }
    }
}
