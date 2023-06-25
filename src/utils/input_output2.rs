use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::marker::PhantomData;
use std::path::Path;

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    // The output is wrapped in a Result to allow matching on errors
    // Returns an Iterator to the Reader of the lines of the file.
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(Debug)]
pub struct Reader<K, V> {
    filename: String,
    sep: char,
    value_type: PhantomData<(K, V)>,
}

impl<K: std::str::FromStr + std::fmt::Debug, V: std::str::FromStr + std::fmt::Debug> Reader<K, V>
where
    K: Eq + std::hash::Hash,
    <K as std::str::FromStr>::Err: std::fmt::Debug,
    <V as std::str::FromStr>::Err: std::fmt::Debug,
{
    pub fn init(file_abs_path: String, separator: char) -> Self {
        Self {
            filename: file_abs_path,
            sep: separator,
            value_type: PhantomData,
        }
    }

    pub fn hashmap_from_txt(&self) -> HashMap<K, V> {
        let mut nb_iter = 0;
        let mut hashmap = HashMap::<K, V>::new();
        match read_lines(self.filename.as_str()) {
            Ok(lines) => {
                for (_, line) in lines.enumerate() {
                    if let Ok(row) = line {
                        let values = row.split(self.sep).collect::<Vec<&str>>();
                        if values.len() >= 2 {
                            hashmap.insert(
                                values[0].parse::<K>().unwrap(),
                                values[1].parse::<V>().unwrap(),
                            );
                        }
                        // println!("{:?}", dg.vertex_edges(&values[0].parse::<usize>().unwrap()));
                        println!("{nb_iter}");
                        nb_iter += 1
                    }
                }
            }
            Err(error) => panic!("{error}"),
        }
        hashmap
    }
}
