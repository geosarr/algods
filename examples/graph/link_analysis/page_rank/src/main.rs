use algods::graph::DirectedGraph;
use rayon::prelude::*;

fn main() {
    let mut graph = DirectedGraph::init(13);
    graph.add_edge(1, 2);
    graph.add_edge(2, 1);
    graph.add_edge(4, 0);
    graph.add_edge(4, 1);
    graph.add_edge(5, 4);
    graph.add_edge(5, 1);
    graph.add_edge(5, 6);
    graph.add_edge(6, 1);
    graph.add_edge(6, 5);
    graph.add_edge(7, 1);
    graph.add_edge(7, 5);
    graph.add_edge(8, 1);
    graph.add_edge(8, 5);
    graph.add_edge(9, 1);
    graph.add_edge(9, 5);
    graph.add_edge(10, 1);
    graph.add_edge(10, 5);
    graph.add_edge(11, 5);
    graph.add_edge(12, 5);

    let nb_vertices = graph.nb_vertices();
    let mut page_ranker = PageRank::init(0.85, nb_vertices);
    println!("{:?}", page_ranker.ranks());
    page_ranker.run(&graph, 1000);
    println!("{:?}", page_ranker.ranks());
}

struct PageRank {
    alpha: f32,
    ranks: Vec<f32>,
}

impl PageRank {
    pub fn init(damp_factor: f32, nb_vertices: usize) -> Self {
        assert!(nb_vertices > 0);
        assert!(damp_factor < 1.0);
        let start_value = 1.0 / (nb_vertices as f32);
        Self {
            alpha: damp_factor,
            ranks: vec![start_value; nb_vertices],
        }
    }
    pub fn ranks(&self) -> &Vec<f32> {
        &self.ranks
    }
    pub fn run(&mut self, graph: &DirectedGraph, nb_iter: usize) {
        let nb_vertices = graph.nb_vertices();
        let nb = nb_vertices as f32;
        for _ in 0..nb_iter {
            let mut pi = self
                .ranks
                .par_iter()
                .enumerate()
                .map(|(v, _)| {
                    let v_out_edges = graph.out_edges(&v);
                    self.ranks
                        .par_iter()
                        .enumerate()
                        .map(|(w, r)| {
                            let w_out_edges = graph.out_edges(&w);
                            let nb_out_edges = w_out_edges.len() as f32;
                            if w_out_edges.par_iter().any(|e| e.to() == &v) {
                                self.alpha * *r / nb_out_edges
                            } else if w_out_edges.is_empty() {
                                self.alpha * *r / nb // stochastic matrix condition
                            } else {
                                (1.0 - self.alpha) * *r / nb // random jumps
                            }
                        })
                        .sum::<f32>()
                })
                .collect::<Vec<f32>>();
            let sum = pi.par_iter().sum::<f32>();
            pi = pi.par_iter().map(|r| r / sum).collect::<Vec<f32>>();
            self.ranks = pi;
        }
    }
}
