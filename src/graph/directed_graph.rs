#[cfg(test)]
mod unit_test;
use crate::graph::{VertexInfo, Weight};
use std::cmp::max;
// use crate::utils::read_lines;
use std::collections::HashSet;
use std::path::Path;

use super::Index;

#[derive(Eq, Hash, PartialEq, Copy, Clone)]
pub struct DiEdge {
    from: usize, // not necessarily useful but keeps the idea of an edge
    to: usize,
}

// impl DiEdge {
//     pub fn init(origin: usize, destination: usize) -> Self {
//         Self {
//             from: origin,
//             to: destination,
//         }
//     }
//     pub fn to(&self) -> &usize {
//         &self.to
//     }
//     pub fn from(&self) -> &usize {
//         &self.from
//     }
// }
// /// Directed Graph based on adjacency-list
// /// ```
// /// use algods::graph::DiDiGraph;
// /// let mut graph = DiGraph::init(3);
// /// graph.add_edge(0, 1);
// /// graph.add_edge(1, 2);
// /// assert_eq!(graph.nb_vertices(), 3);
// /// assert_eq!(graph.nb_edges(), 2);
// /// graph.add_vertex();
// /// assert_eq!(graph.nb_vertices(), 4);
// /// ```
pub struct DiGraph<N>
where
    N: Index,
{
    // implements an adjacency-list graph
    // where vertices have indices 0, ..., nb_objects
    // and each vertex is associated to the vertices it points to
    data: Vec<HashSet<N>>,
    nb_edges: usize,
    nb_vertices: usize,
    in_degree: Vec<usize>,
}
impl<N: Index> Default for DiGraph<N> {
    fn default() -> Self {
        Self::new()
    }
}
impl<N: Index> DiGraph<N> {
    /// Creates an empty graph.
    /// ```
    /// use algods::graph::DiGraph;
    /// let graph = DiGraph::<u128>::new();
    /// assert_eq!(graph.nb_vertices(), 0);
    /// assert_eq!(graph.nb_edges(), 0);
    /// ```
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            nb_edges: 0,
            nb_vertices: 0,
            in_degree: Vec::new(),
        }
    }
    /// Creates a graph with a given number of vertices and without edges.
    /// ```
    /// use algods::graph::DiGraph;
    /// let graph = DiGraph::<u8>::init(10);
    /// assert_eq!(graph.nb_vertices(), 10);
    /// assert_eq!(graph.nb_edges(), 0);
    /// ```
    pub fn init(nb_vertices: usize) -> Self {
        assert!(nb_vertices < N::maximum().to_usize());
        let mut graph = Self::new();
        graph.data = vec![HashSet::new(); nb_vertices];
        graph.nb_vertices = nb_vertices;
        graph.in_degree = vec![0; nb_vertices];
        graph
    }
    //     /// Creates a new graph which has the same vertices but edges reverted.
    //     pub fn reverse(&self) -> Self {
    //         // Gets the reverse graph
    //         let nb_vertices = self.nb_vertices;
    //         let mut rev_graph = Self::init(nb_vertices);
    //         for v in 0..nb_vertices {
    //             let adj_v = self.vertex_edges(&v);
    //             for w in adj_v {
    //                 rev_graph.add_edge(*w, v);
    //             }
    //         }
    //         rev_graph
    //     }
    //     /// Creates a graph from a file
    //     pub fn from_file<P>(filename: P, sep: char, nb_vertices: usize) -> Self
    //     where
    //         P: AsRef<Path>,
    //     {
    //         // Builds a directed graph from a file with edges.
    //         // All the elements of each row should be non
    //         // negative integers separated by the value of the sep
    //         // argument, each row represent one or many edges from the first vertex to
    //         // the other ones. If there is only one value, it will be skipped
    //         let mut nb_iter = 0;
    //         println!("Initializing the graph");
    //         let mut dg = DiGraph::init(nb_vertices);
    //         match read_lines(filename) {
    //             Ok(lines) => {
    //                 for (_, line) in lines.enumerate() {
    //                     if let Ok(row) = line {
    //                         let values = row.split(sep).collect::<Vec<&str>>();
    //                         for i in 1..values.len() {
    //                             dg.add_edge(
    //                                 values[0].parse::<usize>().unwrap(),
    //                                 values[i].parse::<usize>().unwrap(),
    //                             );
    //                         }
    //                         // println!("{:?}", dg.vertex_edges(&values[0].parse::<usize>().unwrap()));
    //                         println!("{nb_iter}");
    //                         nb_iter += 1
    //                     }
    //                 }
    //             }
    //             Err(error) => panic!("{error}"),
    //         }
    //         dg
    // }

    /// Returns the number of edges in the graph.
    /// ```
    /// use algods::graph::DiGraph;
    /// let mut graph = DiGraph::<u8>::new();
    /// graph.add_vertices(3);
    /// graph.add_edge(0, 2);
    /// graph.add_edge(1, 2);
    /// graph.add_edge(2, 0);
    /// assert_eq!(graph.nb_edges(), 3);
    /// ```
    pub fn nb_edges(&self) -> usize {
        // run time complexity O(1)
        self.nb_edges
    }
    /// Returns the number of vertices in the graph.
    /// ```
    /// use algods::graph::DiGraph;
    /// let mut graph = DiGraph::<usize>::init(4);
    /// graph.add_edge(0, 2);
    /// graph.add_edge(2, 2);
    /// assert_eq!(graph.nb_vertices(), 4);
    /// ```
    pub fn nb_vertices(&self) -> usize {
        // run time complexity O(1)
        self.nb_vertices
    }
    /// Adds a new edge to the graph
    /// ```
    /// use algods::graph::DiGraph;
    /// let mut graph = DiGraph::<u8>::init(4);
    /// graph.add_edge(1, 3);
    /// graph.add_edge(1, 2);
    /// graph.add_edge(2, 2);
    /// graph.add_edge(0, 3);
    /// graph.add_edge(1, 2);
    /// graph.add_edge(0, 2);
    /// assert_eq!(graph.nb_edges(), 5);
    /// assert_eq!(graph.nb_vertices(), 4);
    /// ```
    pub fn add_edge(&mut self, vertex_v: N, vertex_w: N) {
        // adds an edge from v to w to the graph
        // run time complexity O(1)
        let v = vertex_v.to_usize();
        let w = vertex_w.to_usize();
        assert!(self.nb_vertices >= max(v, w));
        let w_is_new = self.data[v].insert(vertex_w);
        let ind_w_is_new = usize::from(w_is_new);
        self.in_degree[w] += ind_w_is_new;
        self.nb_edges += ind_w_is_new;
    }
    /// Adds a vertex to the graph.
    /// ```
    /// use algods::graph::DiGraph;
    /// let mut graph = DiGraph::<u8>::new();
    /// graph.add_vertex();
    /// graph.add_vertex();
    /// graph.add_vertex();
    /// assert_eq!(graph.nb_vertices(), 3);
    /// ```
    pub fn add_vertex(&mut self) {
        self.data.push(HashSet::new());
        self.nb_vertices += 1;
    }
    /// Adds some vertices to the graph.
    /// ```
    /// use algods::graph::DiGraph;
    /// let mut graph = DiGraph::<u16>::new();
    /// graph.add_vertices(4);
    /// graph.add_vertex();
    /// graph.add_vertex();
    /// graph.add_edge(0, 1);
    /// graph.add_edge(2, 1);
    /// graph.add_edge(1, 1);
    /// assert_eq!(graph.nb_vertices(), 6);
    /// assert_eq!(graph.nb_edges(), 3);
    /// ```
    pub fn add_vertices(&mut self, nb: usize) {
        let new_size = self.nb_vertices + nb;
        assert!(new_size < N::maximum().to_usize());
        self.data.resize(new_size, HashSet::new());
        self.in_degree.resize(new_size, 0);
        self.nb_vertices += nb;
    }

    /// Gives a reference to the vertices a given vertex points to.
    /// ```
    /// use algods::graph::DiGraph;
    /// use std::collections::HashSet;
    /// let mut graph = DiGraph::<u32>::new();
    /// graph.add_vertices(3);
    /// graph.add_edge(0, 1);
    /// graph.add_edge(2, 1);
    /// graph.add_edge(0, 2);
    /// assert_eq!(graph.out_edges(&0), &HashSet::from([1, 2]));
    /// ```
    pub fn out_edges(&self, vertex: &N) -> &HashSet<N> {
        // gets all the vertices linked to a given vertex v,
        // that is the adjacent vertices of v
        // run time complexity O(1)
        let v = vertex.to_usize();
        &self.data[v]
    }

    /// Returns the vertices pointing to a given vertex
    /// ```
    /// use algods::graph::DiGraph;
    /// use std::collections::HashSet;
    /// let mut graph = DiGraph::<u8>::new();
    /// graph.add_vertices(3);
    /// graph.add_edge(2, 1);
    /// graph.add_edge(0, 0);
    /// graph.add_edge(1, 0);
    /// graph.add_edge(0, 2);
    /// graph.add_edge(2, 0);
    /// assert_eq!(graph.in_edges(&0), HashSet::from([0, 1, 2]));
    /// ```
    pub fn in_edges(&self, vertex: &N) -> HashSet<N> {
        self.data
            .iter()
            .enumerate()
            .filter_map(|(source, adj)| {
                if adj.contains(vertex) {
                    Some(source)
                } else {
                    None
                }
            })
            .map(|source| N::to_index(source))
            .collect::<HashSet<_>>()
    }
    /// Gives the number of vertices a vertex points to.
    /// ```
    /// use algods::graph::DiGraph;
    /// let mut graph = DiGraph::<u32>::new();
    /// graph.add_vertices(3);
    /// graph.add_edge(0, 1);
    /// graph.add_edge(2, 1);
    /// graph.add_edge(0, 2);
    /// assert_eq!(graph.out_degree(&0), 2);
    /// assert_eq!(graph.out_degree(&1), 0);
    /// assert_eq!(graph.out_degree(&2), 1);
    /// ```
    pub fn out_degree(&self, vertex: &N) -> usize {
        // the number of vertices the vertex v points to
        self.out_edges(vertex).len()
    }
    /// Gives the number of vertices pointing to a vertex
    /// ```
    /// use algods::graph::DiGraph;
    /// let mut graph = DiGraph::<u8>::new();
    /// graph.add_vertices(3);
    /// graph.add_edge(2, 1);
    /// graph.add_edge(0, 0);
    /// graph.add_edge(1, 0);
    /// graph.add_edge(0, 2);
    /// graph.add_edge(2, 0);
    /// assert_eq!(graph.in_degree(&0), 3);
    /// assert_eq!(graph.in_degree(&1), 1);
    /// assert_eq!(graph.in_degree(&2), 1);
    /// ```
    pub fn in_degree(&self, vertex: &N) -> usize {
        // gives the number of vertices pointing to vertex v
        self.in_edges(vertex).len()
    }
    /// Gives the integer part of the average number of edges per vertex
    /// # Panic
    /// It panics when there is no vertex in the graph.
    /// ```
    /// use algods::graph::DiGraph;
    /// let mut graph = DiGraph::<u32>::new();
    /// graph.add_vertices(4);
    /// graph.add_edge(0, 1);
    /// graph.add_edge(0, 0);
    /// graph.add_edge(2, 1);
    /// graph.add_edge(0, 3);
    /// graph.add_edge(1, 3);
    /// graph.add_edge(3, 3);
    /// graph.add_edge(2, 0);
    /// assert_eq!(graph.average_degree(), 1);
    /// ```
    pub fn average_degree(&self) -> usize {
        // gets the average number of degree of the graph
        if self.nb_vertices > 0 {
            self.nb_edges / self.nb_vertices
        } else {
            panic!("No vertex in the graph");
        }
    }
    /// Returns the number of vertices pointing to themselves.
    /// ```
    /// use algods::graph::Graph;
    /// let mut graph = Graph::<u8>::new();
    /// graph.add_vertices(3);
    /// graph.add_edge(0, 1);
    /// graph.add_edge(0, 0);
    /// graph.add_edge(1, 1);
    /// graph.add_edge(2, 2);
    /// graph.add_edge(2, 1);
    /// graph.add_edge(2, 0);
    /// assert_eq!(graph.self_loop_number(), 3);
    /// ```
    pub fn self_loop_number(&self) -> usize {
        self.data
            .iter()
            .enumerate()
            .map(|(source, adj)| usize::from(adj.contains(&N::to_index(source))))
            .sum()
    }
}
impl<N: Index> VertexInfo<N> for DiGraph<N> {
    fn vertex_edges(&self, vertex: &N) -> Vec<&N> {
        // gets all the vertices linked to a given vertex v,
        // that is the adjacent vertices of v
        let v = vertex.to_usize();
        self.data[v].iter().collect::<Vec<&N>>()
    }
    fn nb_vertices(&self) -> usize {
        // run time complexity O(1)
        self.nb_vertices
    }
}

// #[derive(Eq, Hash, PartialEq, Copy, Clone, Debug)]
// struct WeightedDiEdge<T>
// where
//     T: Weight,
// {
//     from: usize, // not necessarily useful but keeps the idea of an edge
//     to: usize,
//     weight: T,
// }
// impl<T: Weight> WeightedDiEdge<T> {
//     pub fn init(origin: usize, destination: usize, cost: T) -> Self {
//         Self {
//             from: origin,
//             to: destination,
//             weight: cost,
//         }
//     }
//     pub fn to(&self) -> &usize {
//         &self.to
//     }
//     pub fn from(&self) -> &usize {
//         &self.from
//     }
//     pub fn weight(&self) -> &T {
//         &self.weight
//     }
// }

// pub struct EdgeWeightDiGraph<T>
// where
//     T: Weight,
// {
//     data: Vec<HashSet<WeightedDiEdge<T>>>,
//     nb_edges: usize,
//     nb_vertices: usize,
// }

// impl<T: Weight> Default for EdgeWeightDiGraph<T> {
//     fn default() -> Self {
//         Self::new()
//     }
// }
// impl<T: Weight> EdgeWeightDiGraph<T> {
//     /// Creates a new empty graph.
//     pub fn new() -> Self {
//         Self {
//             data: Vec::new(),
//             nb_edges: 0,
//             nb_vertices: 0,
//         }
//     }
//     /// Creates a new graph with unconnected `nb_objects` objects
//     pub fn init(nb_objects: usize) -> Self {
//         let mut graph = Self::new();
//         graph.nb_vertices = nb_objects;
//         graph.data = Vec::with_capacity(nb_objects);
//         for _ in 0..nb_objects {
//             graph.data.push(HashSet::new());
//         }
//         graph
//     }
//     /// Gives the number of edges
//     pub fn nb_edges(&self) -> usize {
//         // run time complexity O(1)
//         self.nb_edges
//     }
//     /// Gives the number of vertices
//     pub fn nb_vertices(&self) -> usize {
//         // run time complexity O(1)
//         self.nb_vertices
//     }
//     /// Adds a new edge of the graph
//     pub fn add_edge(&mut self, u: usize, v: usize, w: T) {
//         // adds an edge from v to w to the graph
//         // run time complexity O(1)
//         assert!(self.nb_vertices >= std::cmp::max(u, v));
//         let edge = WeightedDiEdge::init(u, v, w);
//         // println!("{edge:?}");
//         let is_new = self.data[u].insert(edge);
//         if is_new {
//             // u --> v is a new directed edge
//             self.nb_edges += 1;
//         }
//     }
//     /// Adds a new vertex to the graph
//     pub fn add_vertex(&mut self) {
//         self.data.push(HashSet::new());
//         self.nb_vertices += 1;
//     }
//     /// Returns an immutable reference to the set of edges
//     pub fn vertex_edges(&self, v: &usize) -> Vec<(&usize, &T)> {
//         // gets all the vertices linked to a given vertex v,
//         // that is the adjacent vertices of v
//         // run time complexity O(1)
//         self.data[*v]
//             .iter()
//             .map(|edge| (edge.to(), edge.weight()))
//             .collect::<Vec<(&usize, &T)>>()
//     }
//     /// Gives the number of vertices a vertex point to
//     pub fn out_degree(&self, v: &usize) -> usize {
//         // the number of vertices the vertex v points to
//         self.vertex_edges(v).len()
//     }
//     /// Gives the number of vertices pointing to a vertex
//     pub fn in_degree(&self, v: &usize) -> usize {
//         // gives the number of vertices pointing to vertex v
//         self.data
//             .iter()
//             .map(|adj| usize::from(adj.iter().any(|edge| v == edge.to())))
//             .sum()
//     }
//     /// Gives the integer part of the average number of edges per vertex
//     pub fn average_degree(&self) -> usize {
//         // gets the average number of degree of the graph
//         if self.nb_vertices > 0 {
//             self.nb_edges / self.nb_vertices
//         } else {
//             panic!("No vertex in the graph");
//         }
//     }
//     /// Returns the number of vertices pointing to themselves
//     pub fn self_loop_number(&self) -> usize {
//         self.data
//             .iter()
//             .map(|adj| usize::from(adj.iter().any(|edge| edge.from() == edge.to())))
//             .sum()
//     }
// }
// impl<T: Weight> VertexInfo for EdgeWeightDiGraph<T> {
//     fn vertex_edges(&self, v: &usize) -> Vec<&usize> {
//         // gets all the vertices linked to a given vertex v,
//         // that is the adjacent vertices of v
//         self.data[*v]
//             .iter()
//             .map(|edge| edge.to())
//             .collect::<Vec<&usize>>()
//     }
//     fn nb_vertices(&self) -> usize {
//         // run time complexity O(1)
//         self.nb_vertices
//     }
// }

// #[derive(Debug, Eq, Hash, PartialEq, Copy, Clone)]
// pub struct FlowEdge<T>
// where
//     T: Weight,
// {
//     from: usize,
//     to: usize,
//     flow: T,
//     capacity: T,
// }

// impl<T: Weight> FlowEdge<T> {
//     pub fn init(origin: usize, destination: usize, f: T, c: T) -> Self {
//         Self {
//             from: origin,
//             to: destination,
//             flow: f,
//             capacity: c,
//         }
//     }

//     pub fn from(&self) -> &usize {
//         &self.from
//     }

//     pub fn to(&self) -> &usize {
//         &self.to
//     }

//     pub fn flow(&self) -> &T {
//         &self.flow
//     }
//     pub fn flow_mut(&mut self) -> &mut T {
//         &mut self.flow
//     }

//     pub fn capacity(&self) -> &T {
//         &self.capacity
//     }
// }

// impl<T: Weight> FlowEdge<T> {
//     pub fn residual_capacity(&self) -> T {
//         self.capacity - self.flow
//     }
//     pub fn add_residual_flow_to(&mut self, vertex: &usize, delta: T) {
//         if vertex == self.from() {
//             self.flow = self.flow - delta;
//         } else if vertex == self.to() {
//             self.flow = self.flow + delta;
//         } else {
//             panic!("Illegal endpoint {vertex}")
//         }
//     }
// }

// pub struct FlowNetwork<T>
// where
//     T: Weight,
// {
//     data: Vec<Vec<FlowEdge<T>>>,
//     nb_edges: usize,
//     nb_vertices: usize,
// }

// impl<T: Weight> FlowNetwork<T> {
//     /// Creates a new empty graph.
//     pub fn new() -> Self {
//         Self {
//             data: Vec::new(),
//             nb_edges: 0,
//             nb_vertices: 0,
//         }
//     }
//     /// Creates a new graph with unconnected `nb_objects` objects
//     pub fn init(nb_objects: usize) -> Self {
//         let mut graph = Self::new();
//         graph.nb_vertices = nb_objects;
//         graph.data = Vec::with_capacity(nb_objects);
//         for _ in 0..nb_objects {
//             graph.data.push(Vec::new());
//         }
//         graph
//     }
//     /// Gives the number of edges
//     pub fn nb_edges(&self) -> usize {
//         // run time complexity O(1)
//         self.nb_edges
//     }
//     /// Gives the number of vertices
//     pub fn nb_vertices(&self) -> usize {
//         // run time complexity O(1)
//         self.nb_vertices
//     }
//     /// Adds a new edge of the graph
//     pub fn add_edge(&mut self, from: usize, to: usize, cap: T) {
//         // adds an edge from v to w to the graph
//         // run time complexity O(1)
//         assert!(self.nb_vertices >= std::cmp::max(from, to));
//         let zero = Weight::zero();
//         let forward_edge = FlowEdge::init(from, to, zero, cap);
//         let backward_edge = FlowEdge::init(to, from, zero, zero);
//         if !self.data[from].contains(&forward_edge) {
//             self.data[from].push(forward_edge);
//             self.data[to].push(backward_edge);
//             self.nb_edges += 1;
//         }
//     }
//     /// Adds a new vertex to the graph
//     pub fn add_vertex(&mut self) {
//         self.data.push(Vec::new());
//         self.nb_vertices += 1;
//     }
//     /// Returns an immutable reference to the set of edges
//     pub fn vertex_edges(&self, v: &usize) -> Vec<&FlowEdge<T>> {
//         // gets all the vertices linked to a given vertex v,
//         // that is the adjacent vertices of v
//         // run time complexity O(1)
//         self.data[*v].iter().collect::<Vec<&FlowEdge<T>>>()
//     }
//     pub fn vertex_edges_mut(&mut self, v: &usize) -> std::slice::IterMut<'_, FlowEdge<T>> {
//         // gets all the vertices linked to a given vertex v,
//         // that is the adjacent vertices of v
//         // run time complexity O(1)
//         self.data[*v].iter_mut()
//     }
//     /// Gives the number of vertices a vertex point to
//     pub fn out_degree(&self, v: &usize) -> usize {
//         // the number of vertices the vertex v points to
//         self.vertex_edges(v).len()
//     }
//     /// Gives the number of vertices pointing to a vertex
//     pub fn in_degree(&self, v: &usize) -> usize {
//         // gives the number of vertices pointing to vertex v
//         self.data
//             .iter()
//             .map(|adj| usize::from(adj.iter().any(|edge| v == edge.to())))
//             .sum()
//     }
//     /// Gives the integer part of the average number of edges per vertex
//     pub fn average_degree(&self) -> usize {
//         // gets the average number of degree of the graph
//         if self.nb_vertices > 0 {
//             self.nb_edges / self.nb_vertices
//         } else {
//             panic!("No vertex in the graph");
//         }
//     }
//     /// Returns the number of vertices pointing to themselves
//     pub fn self_loop_number(&self) -> usize {
//         self.data
//             .iter()
//             .map(|adj| usize::from(adj.iter().any(|edge| edge.from() == edge.to())))
//             .sum()
//     }
// }
