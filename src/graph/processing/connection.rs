use crate::graph::processing::dfs;
use crate::graph::processing::TopologicalSort;
use crate::graph::{DirectedGraph, UndirectedGraph};
pub struct ConnectedComponent {
    // Aims at answering the question are two vertives v and w connected in contant time
    // after preprocessing the graph
    // Identifier of the connected commponent vertices belong to
    id: Vec<usize>,
    // Indicates wether or not a vertex w in the graph is visited
    marked: Vec<bool>,
    // Number of connected components
    nb_cc: usize,
    // Whether or not the algorithm has run
    ran: bool,
}
impl ConnectedComponent {
    pub fn init(nb_vertices: usize) -> Self {
        Self {
            marked: vec![false; nb_vertices],
            id: (0..nb_vertices).collect::<Vec<usize>>(),
            nb_cc: 0,
            ran: false,
        }
    }
    pub fn find_cc(&mut self, graph: &UndirectedGraph) {
        // builds all the connected components from a graph
        let nb = graph.nb_vertices();
        for v in 0..nb {
            if !self.marked[v] {
                // run DFS for each vertex in each component
                dfs(graph, &mut self.marked, &mut self.id, v, v, true, true);
                // here the connected component v is built
                self.nb_cc += 1;
            }
        }
        self.ran = true;
    }
    pub fn connected(&self, v: usize, w: usize) -> Option<bool> {
        // finds out whether or not two vertices are connected
        // run time complexity O(1)
        if !self.marked[v] || !self.marked[w] {
            return None;
        }
        Some(self.id[v] == self.id[w])
    }
    pub fn count(&self) -> usize {
        self.nb_cc
    }
    pub fn is_bipartite(&self) -> Option<bool> {
        if self.ran {
            Some(self.nb_cc == 1)
        } else {
            None
        }
    }
}

pub struct StrongConnectedComponent {
    // Aims at answering the question are two vertives v and w connected in contant time
    // after preprocessing a directed graph
    // Identifier of the strong connected commponents vertices belong to
    id: Vec<usize>,
    // Indicates wether or not a vertex w in the graph is visited
    marked: Vec<bool>,
    // Number of strong connected components
    nb_scc: usize,
}
impl StrongConnectedComponent {
    pub fn init(nb_vertices: usize) -> Self {
        Self {
            marked: vec![false; nb_vertices],
            id: (0..nb_vertices).collect::<Vec<usize>>(),
            nb_scc: 0,
        }
    }
    pub fn find_scc(&mut self, graph: &DirectedGraph) {
        // builds all the string connected components from a directed graph

        // run dfs on the reverse graph
        let nb = graph.nb_vertices();
        let mut topo = TopologicalSort::init(nb);
        topo.depth_first_order(&graph.reverse());
        let order_second_dfs = topo.reverse_postorder();
        // order_second_dfs.reverse();
        for v in 0..nb {
            let v = order_second_dfs[nb - 1 - v];
            if !self.marked[v] {
                // run DFS for each vertex in each component
                dfs(graph, &mut self.marked, &mut self.id, v, v, true, true);
                self.nb_scc += 1;
            }
        }
    }
    pub fn connected(&self, v: usize, w: usize) -> Option<bool> {
        // finds out whether or not two vertices are in the same strong connected component
        // run time complexity O(1)
        if !self.marked[v] || !self.marked[w] {
            return None;
        }
        Some(self.id[v] == self.id[w])
    }
    pub fn count(&self) -> usize {
        self.nb_scc
    }
}
