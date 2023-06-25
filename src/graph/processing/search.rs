mod first_search;
mod shortest_path;
#[cfg(test)]
mod unit_test;
use crate::graph::{EdgeWeightedDigraph, FlowEdge, FlowNetwork, VertexInfo, Weight};
pub use first_search::{bfs, dfs};
pub use shortest_path::{bellman_ford, dijkstra, shortest_path_ewdag};
use std::marker::PhantomData;

pub struct DepthFirstSearch<G> {
    // Indicates whether or not a vertex w in the graph is visited
    marked: Vec<bool>,
    // Indicates what is the previous vertex leading to the current vertex
    // when edge_to[w]=w, then no path is found yet from v to w
    edge_to: Vec<usize>,
    // vertex for which paths are computed
    v: usize,
    // type of the graph
    graph_type: PhantomData<G>,
}
impl<G: VertexInfo> DepthFirstSearch<G> {
    pub fn init(nb_vertices: usize, origin: usize) -> Self {
        Self {
            marked: vec![false; nb_vertices],
            edge_to: (0..nb_vertices).collect::<Vec<usize>>(),
            v: origin,
            graph_type: PhantomData,
        }
    }

    pub fn find_paths(&mut self, graph: &G) {
        // finds all paths from self.v in self.graph
        dfs(
            graph,
            &mut self.marked,
            &mut self.edge_to,
            self.v,
            self.v,
            true,
            false,
        );
    }

    pub fn path_to(&self, w: usize) -> Option<Vec<usize>> {
        // finds the path from v to w
        // run time complexity O(length of the path)
        // can be very time consuming for some applications

        if !self.marked[w] {
            return None;
        }
        let mut path = Vec::<usize>::new();
        let mut x = w;
        while x != self.v {
            path.push(x);
            x = self.edge_to[x];
        }
        path.push(self.v);
        Some(path)
    }
}

pub struct BreadthFirstSearch<G> {
    // Indicates wether or not a vertex w in the graph is visited
    marked: Vec<bool>,
    // Indicates what is the previous vertex leading to the current vertex
    // when edge_to[w]=w, then no path is found yet from v to w
    edge_to: Vec<usize>,
    // Vertex for which paths are computed
    v: usize,
    // type of the graph
    graph_type: PhantomData<G>,
}
impl<G: VertexInfo> BreadthFirstSearch<G> {
    pub fn init(nb_vertices: usize, origin: usize) -> Self {
        Self {
            marked: vec![false; nb_vertices],
            edge_to: (0..nb_vertices).collect::<Vec<usize>>(),
            v: origin,
            graph_type: PhantomData,
        }
    }

    pub fn find_paths(&mut self, graph: &G) {
        // finds all reachable vertices from w
        bfs(graph, &mut self.marked, &mut self.edge_to, self.v);
    }

    pub fn path_to(&self, w: usize) -> Option<Vec<usize>> {
        // finds the path from v to w
        // run time complexity O(length of the path)
        // computes shortest paths
        if !self.marked[w] {
            return None;
        }
        let mut path = Vec::<usize>::new();
        let mut x = w;
        while x != self.v {
            path.push(x);
            x = self.edge_to[x];
        }
        path.push(self.v);
        Some(path)
    }
}

pub enum ShortestPathAlgo {
    Dijkstra,
    SpDag,
    BellmanFord,
}
impl Default for ShortestPathAlgo {
    fn default() -> Self {
        Self::Dijkstra
    }
}
pub struct ShortestPath<T> {
    // the source vertex from where the shortest
    // paths are computed
    source: usize,
    // the algorithm used to compute the shortest paths
    algo: ShortestPathAlgo,
    // stores the length of the shortest path from
    // the source to an edge
    dist_to: Vec<T>,
    // stores the vertex that is the closest
    // to an edge in the shortest path
    edge_to: Vec<usize>,
}
impl<T: Weight + Clone + std::hash::Hash> ShortestPath<T> {
    pub fn init(from: usize, algorithm: ShortestPathAlgo, nb_vertices: usize) -> Self {
        Self {
            source: from,
            algo: algorithm,
            dist_to: vec![Weight::max(); nb_vertices],
            edge_to: vec![usize::MAX; nb_vertices],
        }
    }
}

impl<T> ShortestPath<T> {
    pub fn dist_to(&self, v: usize) -> &T {
        &self.dist_to[v]
    }
    pub fn edge_to(&self, v: usize) -> usize {
        self.edge_to[v]
    }
}
impl<T: Eq + Weight> ShortestPath<T> {
    pub fn path_to(&self, v: usize) -> Option<Vec<usize>> {
        if self.dist_to[v] == Weight::max() {
            return None;
        }
        let mut path = Vec::new();
        let mut origin = v;
        while origin != self.source {
            path.push(origin);
            origin = self.edge_to[origin];
        }
        path.push(self.source);
        Some(path)
    }
}

impl<T: Ord + Weight + std::ops::Add<Output = T> + std::hash::Hash> ShortestPath<T> {
    pub fn find_paths(&mut self, graph: &EdgeWeightedDigraph<T>) {
        match self.algo {
            ShortestPathAlgo::Dijkstra => {
                dijkstra(graph, self.source, &mut self.edge_to, &mut self.dist_to);
            }
            ShortestPathAlgo::SpDag => {
                shortest_path_ewdag(graph, self.source, &mut self.edge_to, &mut self.dist_to);
            }
            ShortestPathAlgo::BellmanFord => {
                bellman_ford(graph, self.source, &mut self.edge_to, &mut self.dist_to);
            }
        }
    }
}
