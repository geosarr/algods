#[cfg(test)]
mod unit_test;
use algods::graph::{processing::BreadthFirstSearch, DirectedGraph};
use algods::utils::Reader2;
use clap::Parser;
use std::collections::{HashMap, HashSet};
use std::io;
type SapOutput = Option<(usize, usize, usize, Vec<usize>, Vec<usize>)>;

#[derive(Parser)]
#[command(name = "wordnet")]
#[command(author = "Georges Mbissane SARR <georgesmbissanes@gmail.com>")]
#[command(version = "0.0.0")]
#[command(about = "Runs some algorithms on a wordnet", long_about = None)]

struct Cli {
    /// Path to the hypernyms .txt file. In this file, line i (counting from 0)
    /// contains the hypernyms of synset i. The first field is the synset id,
    /// which is always the integer i; subsequent fields are the id numbers of the synset's hypernyms
    #[arg(short = 'H', long)]
    hyp_path: String,

    /// Separator of the hypernyms file
    #[arg(short = 'd', long)]
    hyp_sep: char,

    /// Path to the synsets .txt file. Line i of the file (counting from 0) contains
    /// the information for synset i. The first field is the synset id, which is
    /// always the integer i; the second field is the synonym set (or synset)
    #[arg(short = 'S', long)]
    syn_path: String,

    /// Separator of the synsets file
    #[arg(short = 's', long)]
    syn_sep: char,

    /// Maximum number of hypernyms to consider
    #[arg(short, long)]
    max_hyp_num: usize,
}

pub fn main() {
    let cli = Cli::parse();
    let wordnet = Wordnet::from_file(
        cli.syn_path.as_str(),
        cli.hyp_path.as_str(),
        cli.hyp_sep,
        cli.syn_sep,
        cli.max_hyp_num,
    );

    loop {
        let mut msg = String::new();

        println!("\nPlease enter two words separated by a whitespace, press Ctrl + C to exit");

        io::stdin()
            .read_line(&mut msg)
            .expect("Failed to read line");

        let mut nouns = msg.split_whitespace();

        let a = nouns.next().expect("failed to get the first word").trim();
        let b = nouns.next().expect("failed to get the second word").trim();

        let (dist, path) = wordnet.sap_distance(a, b);
        println!("Shortest ancestral path betwen {a} and {b} is {path:?} with distance = {dist:?}");
    }
}

/// This struct implements a basic wordnet structure and some processing algorithm
pub struct Wordnet {
    hypernym_graph: DirectedGraph,
    synset: HashMap<usize, String>,
}
impl Wordnet {
    pub fn init(synsets: HashMap<usize, String>, graph: DirectedGraph) -> Self {
        Self {
            hypernym_graph: graph,
            synset: synsets,
        }
    }
    pub fn from_file(
        synsets_file: &str,
        hypernyms_file: &str,
        sep1: char,
        sep2: char,
        nb_vertices: usize,
    ) -> Self {
        let hypernyms = DirectedGraph::from_file(hypernyms_file, sep1, nb_vertices);
        let synsets =
            Reader2::<usize, String>::init(synsets_file.to_string(), sep2).hashmap_from_txt();
        Self::init(synsets, hypernyms)
    }
    pub fn nouns(&self) -> Vec<&String> {
        // returns all Wordnet nouns
        self.synset.values().collect::<Vec<&String>>()
    }
    pub fn is_noun(&self, word: &str) -> bool {
        // indicates whether or not word is a Wordnet noun
        self.synset
            .iter()
            .any(|(_, v)| v.split(' ').any(|x| x == word))
    }
    fn synsets_of_noun<'a>(&'a self, noun: &str) -> HashSet<Option<&'a usize>> {
        return self
            .synset
            .iter()
            .map(|(k, v)| {
                if v.split(' ').any(|x| x == noun) {
                    Some(k)
                } else {
                    None
                }
            })
            .collect::<HashSet<_>>();
    }
    pub fn sap_distance(
        &self,
        noun_a: &str,
        noun_b: &str,
    ) -> (Option<usize>, Option<Vec<&String>>) {
        // shortest ancestor path distance along wiith the path

        let synset_a = self.synsets_of_noun(noun_a);
        let synset_b = self.synsets_of_noun(noun_b);
        let mut min_distance = 2 * self.hypernym_graph.nb_vertices();
        let mut distance: Option<usize> = None;
        let mut path: Option<Vec<&String>> = None;
        let sap = ShortestAncestralPath::new();
        if !((synset_a.len() == 1 && synset_a.contains(&None))
            || (synset_b.len() == 1 && synset_b.contains(&None)))
        {
            // both words are in Wordnet
            for a in synset_a.iter().flatten() {
                for b in synset_b.iter().flatten() {
                    if let Some(ancestor) = sap.ancestor(&self.hypernym_graph, **a, **b) {
                        if ancestor.1 + ancestor.2 < min_distance {
                            min_distance = ancestor.1 + ancestor.2;
                            // computing the sap distance
                            distance = Some(ancestor.1 + ancestor.2);
                            // building the path
                            let mut half_path_one = ancestor.3;
                            half_path_one.reverse();
                            half_path_one.pop();
                            let mut half_path_two = ancestor.4;
                            half_path_one.append(&mut half_path_two);
                            path = Some(
                                half_path_one
                                    .iter()
                                    .map(|e| &self.synset[e])
                                    .collect::<Vec<&String>>(),
                            );
                        }
                    }
                }
            }
        }
        (distance, path)
    }
}

struct ShortestAncestralPath {
    // An ancestral path between two vertices v and w in a digraph is a directed path from v to a common ancestor x,
    // together with a directed path from w to the same ancestor x. A shortest ancestral path is an ancestral path
    // of minimum total length. We refer to the common ancestor in a shortest ancestral path as a shortest common ancestor.
    // Note also that an ancestral path is a path, but not a directed path.
    // (from https://coursera.cs.princeton.edu/algs4/assignments/wordnet/specification.php)
}
impl ShortestAncestralPath {
    pub fn new() -> Self {
        Self {}
    }
    pub fn ancestor(&self, graph: &DirectedGraph, v: usize, w: usize) -> SapOutput {
        // a common ancestor of v and w that participates in a shortest ancestral path, if any;
        // along with the length of the paths from v to the ancestor and from w to the ancestor
        // run time complexity O(number_vertices + number_edges)
        // space complexity O(number_vertices)

        // first get the shortest paths from v to any reachable vertex from v
        // and do the same for w
        let nb_vertices = graph.nb_vertices();
        let mut bfs_v = BreadthFirstSearch::init(nb_vertices, v);
        let mut bfs_w = BreadthFirstSearch::init(nb_vertices, w);
        bfs_v.find_paths(graph);
        bfs_w.find_paths(graph);

        // find a valid common ancestor
        let mut min_len = 2 * nb_vertices; // minimum ancestral path length
        let mut ancestor: SapOutput = None;
        for u in 0..nb_vertices {
            // find out if a given vertex u in the graph is at the intersection
            // of two paths, one from v and the other from w
            let path_v_u = bfs_v.path_to(u);
            let path_w_u = bfs_w.path_to(u);
            if let Some(path_v) = path_v_u {
                if let Some(path_w) = path_w_u {
                    // u is a candidate ancestor
                    let dist_v_u = path_v.len() - 1;
                    let dist_w_u = path_w.len() - 1;
                    if dist_v_u + dist_w_u < min_len {
                        // u is the best current candidate ancestor
                        min_len = dist_v_u + dist_w_u;
                        ancestor = Some((u, dist_v_u, dist_w_u, path_v, path_w));
                    }
                }
            }
        }
        ancestor
    }
}
