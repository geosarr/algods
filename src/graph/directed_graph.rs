#[cfg(test)]
mod unit_test;
use crate::graph::{VertexInfo, Weight};
use std::cmp::max;
// use crate::utils::read_lines;
use std::collections::HashSet;

use super::Index;

/// Directed Graph based on adjacency-list
/// ```
/// use algods::graph::DiGraph;
/// let mut graph = DiGraph::<u8>::init(3);
/// graph.add_edge(0, 1);
/// graph.add_edge(1, 2);
/// assert_eq!(graph.nb_vertices(), 3);
/// assert_eq!(graph.nb_edges(), 2);
/// graph.add_vertex();
/// assert_eq!(graph.nb_vertices(), 4);
/// graph.add_edge(0, 0);
/// assert_eq!(graph.self_loop_number(), 1);
/// ```
#[derive(Debug, PartialEq, Eq)]
pub struct DiGraph<N>
where
    N: Index,
{
    // implements an adjacency-list graph
    // where vertices have indices 0, ..., nb_objects
    // and each vertex is associated to the vertices it points to
    out_edges: Vec<HashSet<N>>,
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
            out_edges: Vec::new(),
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
        graph.out_edges = vec![HashSet::new(); nb_vertices];
        graph.nb_vertices = nb_vertices;
        graph.in_degree = vec![0; nb_vertices];
        graph
    }
    /// Creates a new graph from a `Vec` of edges.
    /// ```
    /// use algods::graph::DiGraph;
    /// let mut graph = DiGraph::<u8>::from_vec(vec![(0, 0), (1, 0), (0, 2), (3, 1), (2, 3)]);
    /// assert_eq!(graph.nb_vertices(), 4);
    /// assert_eq!(graph.nb_edges(), 5);
    /// ```
    pub fn from_vec(edges: Vec<(N, N)>) -> Self {
        let mut graph = Self::new();
        let nb_edges = edges.len();
        for edge in edges.iter().take(nb_edges) {
            let source = edge.0;
            let target = edge.1;
            let max_vertex = max(source, target).to_usize();
            if max_vertex >= graph.nb_vertices {
                graph.add_vertices(max_vertex - graph.nb_vertices + 1);
            }
            graph.add_edge(source, target);
            graph.in_degree[target.to_usize()] += 1;
        }
        graph
    }
    /// Creates a new graph which has the same vertices but edges reverted.
    /// ```
    /// use algods::graph::DiGraph;
    /// let mut graph = DiGraph::<u8>::init(4);
    /// graph.add_edge(0, 0);
    /// graph.add_edge(0, 1);
    /// graph.add_edge(0, 2);
    /// graph.add_edge(1, 3);
    /// graph.add_edge(3, 2);
    /// let expected_reverse_graph =
    /// DiGraph::<u8>::from_vec(vec![(0, 0), (1, 0), (2, 0), (3, 1), (2, 3)]);
    /// assert_eq!(graph.reverse(), expected_reverse_graph);
    /// assert_eq!(expected_reverse_graph.in_degree(&0), 3);
    /// ```
    pub fn reverse(&self) -> Self {
        // Gets the reverse graph
        let mut rev_graph = Self::init(self.nb_vertices);
        for v in 0..self.nb_vertices {
            let vertex_v = N::to_vertex(v);
            let adj_v = self.out_edges(&vertex_v);
            for vertex_w in adj_v {
                rev_graph.add_edge(*vertex_w, vertex_v);
                rev_graph.in_degree[v] += 1;
            }
        }
        rev_graph
    }
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
    pub fn add_edge(&mut self, source: N, target: N) {
        // adds an edge from source to target to the graph
        // run time complexity O(1)
        let s = source.to_usize();
        let t = target.to_usize();
        assert!(self.nb_vertices >= max(s, t));
        let target_is_new = self.out_edges[s].insert(target);
        let ind_t_is_new = usize::from(target_is_new);
        self.in_degree[t] += ind_t_is_new;
        self.nb_edges += ind_t_is_new;
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
        self.out_edges.push(HashSet::new());
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
        self.out_edges.resize(new_size, HashSet::new());
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
        &self.out_edges[v]
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
        self.out_edges
            .iter()
            .enumerate()
            .filter_map(|(source, adj)| {
                if adj.contains(vertex) {
                    Some(source)
                } else {
                    None
                }
            })
            .map(|source| N::to_vertex(source))
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
    /// graph.add_edge(2, 2);
    /// assert_eq!(graph.out_degree(&0), 2);
    /// assert_eq!(graph.out_degree(&1), 0);
    /// assert_eq!(graph.out_degree(&2), 2);
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
    /// # Panics
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
    /// use algods::graph::DiGraph;
    /// let mut graph = DiGraph::<u8>::new();
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
        self.out_edges
            .iter()
            .enumerate()
            .map(|(source, adj)| usize::from(adj.contains(&N::to_vertex(source))))
            .sum()
    }
}
impl<N: Index> VertexInfo<N> for DiGraph<N> {
    fn vertex_edges(&self, vertex: &N) -> Vec<&N> {
        // gets all the vertices linked to a given vertex v,
        // that is the adjacent vertices of v
        let v = vertex.to_usize();
        self.out_edges[v].iter().collect::<Vec<&N>>()
    }
    fn nb_vertices(&self) -> usize {
        // run time complexity O(1)
        self.nb_vertices
    }
}

#[derive(Eq, Hash, PartialEq, Copy, Clone, Debug)]
pub struct WeightedDiEdge<N, W>
where
    N: Index,
    W: Weight,
{
    from: N, // not necessarily useful but keeps the idea of an edge
    to: N,
    weight: W,
}
impl<N: Index, W: Weight> WeightedDiEdge<N, W> {
    ///  Creates a weighted edge from origin to destination.
    /// ```
    /// use algods::graph::{WeightedDiEdge};
    /// let edge = WeightedDiEdge::<u8, u16>::init(0, 1, 4);
    /// assert_eq!(edge.from(), &0);
    /// assert_eq!(edge.to(), &1);
    /// assert_eq!(edge.weight(), &4);
    /// let self_loop_edge = WeightedDiEdge::<u8, i8>::init(1, 1, -4);
    /// assert_eq!(self_loop_edge.from(), &1);
    /// assert_eq!(self_loop_edge.to(), &1);
    /// assert_eq!(self_loop_edge.weight(), &-4);
    /// ```
    pub fn init(origin: N, destination: N, weight: W) -> Self {
        Self {
            from: origin,
            to: destination,
            weight,
        }
    }
    ///  Gives the destination vertex of the edge.
    /// ```
    /// use algods::graph::{WeightedDiEdge};
    /// let edge = WeightedDiEdge::<u8, u16>::init(9, 2, 6);
    /// assert_eq!(edge.to(), &2);
    /// ```
    pub fn to(&self) -> &N {
        &self.to
    }
    ///  Gives the origin vertex of the edge.
    /// ```
    /// use algods::graph::{WeightedDiEdge};
    /// let edge = WeightedDiEdge::<u8, isize>::init(0, 2, -1);
    /// assert_eq!(edge.from(), &0);
    /// ```
    pub fn from(&self) -> &N {
        &self.from
    }
    ///  Gives the weight of the edge.
    /// ```
    /// use algods::graph::{WeightedDiEdge};
    /// let edge = WeightedDiEdge::<u8, u8>::init(7, 2, 10);
    /// assert_eq!(edge.weight(), &10);
    /// ```
    pub fn weight(&self) -> &W {
        &self.weight
    }
}

#[derive(Clone, Debug)]
pub struct EdgeWeightedDiGraph<N, W>
where
    N: Index,
    W: Weight,
{
    out_edges: Vec<HashSet<WeightedDiEdge<N, W>>>,
    nb_edges: usize,
    nb_vertices: usize,
    in_degree: Vec<usize>,
}

impl<N: Index, W: Weight> Default for EdgeWeightedDiGraph<N, W> {
    fn default() -> Self {
        Self::new()
    }
}
impl<N: Index, W: Weight> EdgeWeightedDiGraph<N, W> {
    /// Creates an empty graph.
    /// ```
    /// use algods::graph::EdgeWeightedDiGraph;
    /// let graph = EdgeWeightedDiGraph::<u8, i16>::new();
    /// assert_eq!(graph.nb_vertices(), 0);
    /// assert_eq!(graph.nb_edges(), 0);
    /// ```
    pub fn new() -> Self {
        Self {
            out_edges: Vec::new(),
            nb_edges: 0,
            nb_vertices: 0,
            in_degree: Vec::new(),
        }
    }
    /// Creates a graph with a given number of vertices and without edges.
    /// ```
    /// use algods::graph::EdgeWeightedDiGraph;
    /// let graph = EdgeWeightedDiGraph::<u16, usize>::init(10);
    /// assert_eq!(graph.nb_vertices(), 10);
    /// assert_eq!(graph.nb_edges(), 0);
    /// ```
    pub fn init(nb_vertices: usize) -> Self {
        assert!(nb_vertices < N::maximum().to_usize());
        let mut graph = Self::new();
        graph.out_edges = vec![HashSet::new(); nb_vertices];
        graph.nb_vertices = nb_vertices;
        graph.in_degree = vec![0; nb_vertices];
        graph
    }
    /// Creates a new graph from a `Vec` of edges.
    /// ```
    /// use algods::graph::EdgeWeightedDiGraph;
    /// let mut graph = EdgeWeightedDiGraph::<u8, isize>::from_vec(vec![
    ///     (0, 0, 0),
    ///     (1, 0, -1),
    ///     (0, 2, 3),
    ///     (3, 1, 10),
    ///     (2, 3, 5),
    /// ]);
    /// assert_eq!(graph.nb_vertices(), 4);
    /// assert_eq!(graph.nb_edges(), 5);
    /// ```
    pub fn from_vec(edges: Vec<(N, N, W)>) -> Self {
        let mut graph = Self::new();
        let nb_edges = edges.len();
        for edge in edges.iter().take(nb_edges) {
            let source = edge.0;
            let target = edge.1;
            let weight = edge.2;
            let max_vertex = max(source, target).to_usize();
            if max_vertex >= graph.nb_vertices {
                graph.add_vertices(max_vertex - graph.nb_vertices + 1);
            }
            graph.add_edge(source, target, weight);
            graph.in_degree[target.to_usize()] += 1;
        }
        graph
    }
    /// Returns the number of edges in the graph.
    /// ```
    /// use algods::graph::EdgeWeightedDiGraph;
    /// let mut graph = EdgeWeightedDiGraph::<u8, u8>::new();
    /// graph.add_vertices(3);
    /// graph.add_edge(0, 2, 0);
    /// graph.add_edge(1, 2, 0);
    /// graph.add_edge(2, 0, 0);
    /// assert_eq!(graph.nb_edges(), 3);
    /// ```
    pub fn nb_edges(&self) -> usize {
        // run time complexity O(1)
        self.nb_edges
    }
    /// Returns the number of vertices in the graph.
    /// ```
    /// use algods::graph::EdgeWeightedDiGraph;
    /// let mut graph = EdgeWeightedDiGraph::<usize, usize>::init(4);
    /// graph.add_edge(0, 2, 8);
    /// graph.add_edge(2, 2, 7);
    /// assert_eq!(graph.nb_vertices(), 4);
    /// ```
    pub fn nb_vertices(&self) -> usize {
        // run time complexity O(1)
        self.nb_vertices
    }
    /// Adds a new edge to the graph
    /// ```
    /// use algods::graph::EdgeWeightedDiGraph;
    /// let mut graph = EdgeWeightedDiGraph::<u8, u16>::init(4);
    /// graph.add_edge(1, 3, 0);
    /// graph.add_edge(1, 2, 2);
    /// graph.add_edge(2, 2, 2);
    /// graph.add_edge(0, 3, 0);
    /// graph.add_edge(1, 2, 2);
    /// graph.add_edge(1, 2, 18);
    /// graph.add_edge(0, 2, 20);
    /// // Remark that there 2 different edges between 1 and 2
    /// assert_eq!(graph.nb_edges(), 6);
    /// assert_eq!(graph.nb_vertices(), 4);
    /// ```
    pub fn add_edge(&mut self, source: N, target: N, weight: W) {
        // adds an edge from u to v with weight w to the graph
        // run time complexity O(1)
        let s = source.to_usize();
        let t = target.to_usize();
        assert!(self.nb_vertices >= std::cmp::max(s, t));
        let edge = WeightedDiEdge::init(source, target, weight);
        let target_is_new = self.out_edges[s].insert(edge);
        let ind_t_is_new = usize::from(target_is_new);
        self.in_degree[t] += ind_t_is_new;
        self.nb_edges += ind_t_is_new;
    }
    /// Adds some vertices to the graph.
    /// ```
    /// use algods::graph::EdgeWeightedDiGraph;
    /// let mut graph = EdgeWeightedDiGraph::<u16, usize>::new();
    /// graph.add_vertices(4);
    /// graph.add_vertex();
    /// graph.add_vertex();
    /// graph.add_edge(0, 1, 2);
    /// graph.add_edge(2, 1, 0);
    /// graph.add_edge(1, 1, 7);
    /// assert_eq!(graph.nb_vertices(), 6);
    /// assert_eq!(graph.nb_edges(), 3);
    /// ```
    pub fn add_vertices(&mut self, nb: usize) {
        let new_size = self.nb_vertices + nb;
        assert!(new_size < N::maximum().to_usize());
        self.out_edges.resize(new_size, HashSet::new());
        self.in_degree.resize(new_size, 0);
        self.nb_vertices += nb;
    }
    /// Adds a vertex to the graph.
    /// ```
    /// use algods::graph::EdgeWeightedDiGraph;
    /// let mut graph = EdgeWeightedDiGraph::<usize, isize>::new();
    /// graph.add_vertex();
    /// graph.add_vertex();
    /// graph.add_vertex();
    /// assert_eq!(graph.nb_vertices(), 3);
    /// ```
    pub fn add_vertex(&mut self) {
        self.out_edges.push(HashSet::new());
        self.nb_vertices += 1;
    }
    /// Gives a reference to the vertices a given vertex points to.
    /// ```
    /// use algods::graph::{EdgeWeightedDiGraph, WeightedDiEdge};
    /// use std::collections::HashSet;
    /// let mut graph = EdgeWeightedDiGraph::<u32, isize>::new();
    /// graph.add_vertices(3);
    /// graph.add_edge(0, 1, 0);
    /// graph.add_edge(2, 1, 3);
    /// graph.add_edge(0, 2, -1);
    /// let edge_0_1 = WeightedDiEdge::init(0, 1, 0);
    /// let edge_0_2 = WeightedDiEdge::init(0, 2, -1);
    /// assert_eq!(graph.out_edges(&0), &HashSet::from([edge_0_1, edge_0_2]));
    /// ```
    pub fn out_edges(&self, vertex: &N) -> &HashSet<WeightedDiEdge<N, W>> {
        // gets all the vertices linked to a given vertex v,
        // that is the adjacent vertices of v
        // run time complexity O(1)
        let v = vertex.to_usize();
        &self.out_edges[v]
    }
    /// Returns the vertices pointing to a given vertex
    /// ```
    /// use algods::graph::{EdgeWeightedDiGraph, WeightedDiEdge};
    /// use std::collections::HashSet;
    /// let mut graph = EdgeWeightedDiGraph::<u8, i16>::new();
    /// graph.add_vertices(3);
    /// graph.add_edge(2, 1, 0);
    /// graph.add_edge(0, 0, -20);
    /// graph.add_edge(1, 0, 10);
    /// graph.add_edge(0, 2, 8);
    /// graph.add_edge(2, 0, -3);
    /// let edge_0_0 = WeightedDiEdge::init(0, 0, -20);
    /// let edge_1_0 = WeightedDiEdge::init(1, 0, 10);
    /// let edge_2_0 = WeightedDiEdge::init(2, 0, -3);
    /// assert_eq!(graph.in_edges(&0), HashSet::from([&edge_0_0, &edge_1_0, &edge_2_0]));
    /// ```
    pub fn in_edges(&self, vertex: &N) -> HashSet<&WeightedDiEdge<N, W>> {
        self.out_edges
            .iter()
            .filter_map(|adj| adj.iter().find(|&edge| edge.to() == vertex))
            .collect::<HashSet<_>>()
    }
    /// Gives the number of vertices a vertex points to.
    /// ```
    /// use algods::graph::EdgeWeightedDiGraph;
    /// let mut graph = EdgeWeightedDiGraph::<u32, i8>::new();
    /// graph.add_vertices(3);
    /// graph.add_edge(0, 1, 0);
    /// graph.add_edge(2, 1, 0);
    /// graph.add_edge(0, 2, 0);
    /// graph.add_edge(2, 2, 1);
    /// assert_eq!(graph.out_degree(&0), 2);
    /// assert_eq!(graph.out_degree(&1), 0);
    /// assert_eq!(graph.out_degree(&2), 2);
    /// ```
    pub fn out_degree(&self, vertex: &N) -> usize {
        // the number of vertices the vertex v points to
        let v = vertex.to_usize();
        self.out_edges[v].len()
    }
    /// Gives the number of vertices pointing to a vertex
    /// ```
    /// use algods::graph::EdgeWeightedDiGraph;
    /// let mut graph = EdgeWeightedDiGraph::<u8, u8>::new();
    /// graph.add_vertices(3);
    /// graph.add_edge(2, 1, 1);
    /// graph.add_edge(0, 0, 5);
    /// graph.add_edge(1, 0, 7);
    /// graph.add_edge(0, 2, 2);
    /// graph.add_edge(2, 0, 9);
    /// assert_eq!(graph.in_degree(&0), 3);
    /// assert_eq!(graph.in_degree(&1), 1);
    /// assert_eq!(graph.in_degree(&2), 1);
    /// ```
    pub fn in_degree(&self, vertex: &N) -> usize {
        // gives the number of vertices pointing to vertex v
        let v = vertex.to_usize();
        self.in_degree[v]
    }
    /// Gives the integer part of the average number of edges per vertex
    /// # Panics
    /// It panics when there is no vertex in the graph.
    /// ```
    /// use algods::graph::EdgeWeightedDiGraph;
    /// let mut graph = EdgeWeightedDiGraph::<u32, usize>::new();
    /// graph.add_vertices(4);
    /// graph.add_edge(0, 1, 0);
    /// graph.add_edge(0, 0, 10);
    /// graph.add_edge(2, 1, 4);
    /// graph.add_edge(0, 3, 6);
    /// graph.add_edge(1, 3, 4);
    /// graph.add_edge(3, 3, 5);
    /// graph.add_edge(2, 0, 0);
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
    /// use algods::graph::EdgeWeightedDiGraph;
    /// let mut graph = EdgeWeightedDiGraph::<usize, isize>::new();
    /// graph.add_vertices(3);
    /// graph.add_edge(0, 1, 0);
    /// graph.add_edge(0, 0, 1);
    /// graph.add_edge(1, 1, 2);
    /// graph.add_edge(2, 2, 3);
    /// graph.add_edge(2, 1, 4);
    /// graph.add_edge(2, 0, 7);
    /// assert_eq!(graph.self_loop_number(), 3);
    /// ```
    pub fn self_loop_number(&self) -> usize {
        self.out_edges
            .iter()
            .map(|adj| usize::from(adj.iter().any(|edge| edge.from() == edge.to())))
            .sum()
    }
}
impl<N: Index, W: Weight> VertexInfo<N> for EdgeWeightedDiGraph<N, W> {
    fn vertex_edges(&self, vertex: &N) -> Vec<&N> {
        // gets all the vertices linked to a given vertex v,
        // that is the adjacent vertices of v
        let v = vertex.to_usize();
        self.out_edges[v]
            .iter()
            .map(|edge| edge.to())
            .collect::<Vec<&N>>()
    }
    fn nb_vertices(&self) -> usize {
        // run time complexity O(1)
        self.nb_vertices
    }
}

#[derive(Debug, Eq, Hash, PartialEq, Copy, Clone)]
pub struct FlowEdge<N, W>
where
    N: Index,
    W: Weight,
{
    from: N,
    to: N,
    flow: W,
    capacity: W,
}

impl<N: Index, W: Weight> FlowEdge<N, W> {
    /// Creates a weighted edge from origin to destination.
    /// ```
    /// use algods::graph::FlowEdge;
    /// let edge = FlowEdge::<u8, u16>::init(4, 2, 5, 10);
    /// assert_eq!(edge.from(), &4);
    /// assert_eq!(edge.to(), &2);
    /// assert_eq!(edge.flow(), &5);
    /// assert_eq!(edge.capacity(), &10);
    /// ```
    pub fn init(from: N, to: N, flow: W, capacity: W) -> Self {
        assert!(flow <= capacity);
        Self {
            from,
            to,
            flow,
            capacity,
        }
    }
    /// Gives the origin vertex of the edge.
    /// ```
    /// use algods::graph::FlowEdge;
    /// let edge = FlowEdge::<u8, usize>::init(0, 2, 1, 4);
    /// assert_eq!(edge.from(), &0);
    /// ```
    pub fn from(&self) -> &N {
        &self.from
    }
    /// Gives the destination vertex of the edge.
    /// ```
    /// use algods::graph::FlowEdge;
    /// let edge = FlowEdge::<u8, u16>::init(0, 2, 6, 17);
    /// assert_eq!(edge.to(), &2);
    /// ```
    pub fn to(&self) -> &N {
        &self.to
    }
    /// Gives the flow of the edge.
    /// ```
    /// use algods::graph::FlowEdge;
    /// let edge = FlowEdge::<u8, u8>::init(7, 2, 4, 5);
    /// assert_eq!(edge.flow(), &4);
    /// ```
    pub fn flow(&self) -> &W {
        &self.flow
    }
    /// Gives a mutable reference to the flow of the edge.
    /// ```
    /// use algods::graph::FlowEdge;
    /// let mut edge = FlowEdge::<u8, u8>::init(7, 2, 3, 15);
    /// let mut flow = edge.flow_mut();
    /// *flow = 9;
    /// assert_eq!(edge.flow(), &9);
    /// ```
    pub fn flow_mut(&mut self) -> &mut W {
        &mut self.flow
    }
    /// Gives the capacity of the edge.
    /// ```
    /// use algods::graph::FlowEdge;
    /// let edge = FlowEdge::<u8, u8>::init(7, 2, 4, 5);
    /// assert_eq!(edge.capacity(), &5);
    /// ```
    pub fn capacity(&self) -> &W {
        &self.capacity
    }
    /// Returns the difference between the capacity and the flow of the edge.
    /// ```
    /// use algods::graph::FlowEdge;
    /// let edge = FlowEdge::<u8, u16>::init(4, 2, 5, 8);
    /// assert_eq!(edge.residual_capacity(), 3);
    /// ```
    pub fn residual_capacity(&self) -> W {
        self.capacity - self.flow
    }
    /// Mutates the flow depending on `vertex` argument. That is:
    /// * If `vertex` is the origin/source of the edge (i.e `edge.from() == vertex`), then it reduces the flow by `delta`.
    /// * If `vertex` is the destination of the edge (i.e `edge.to() == vertex`), then it adds to the flow `delta`.
    /// # Panics
    /// When the `vertex` is neither the source nor the destination of the edge then it panics.
    /// ```
    /// use algods::graph::FlowEdge;
    /// let mut edge = FlowEdge::<u8, u16>::init(4, 2, 5, 8);
    /// edge.add_residual_flow_to(&4, 2);
    /// assert_eq!(edge.flow(), &3);
    /// ```
    pub fn add_residual_flow_to(&mut self, vertex: &N, delta: W) {
        if vertex == self.from() {
            self.flow = self.flow - delta;
        } else if vertex == self.to() {
            self.flow = self.flow + delta;
        } else {
            let v = vertex.to_usize();
            panic!("Illegal endpoint {v}")
        }
    }
}

pub struct FlowNetwork<N, W>
where
    N: Index,
    W: Weight,
{
    out_edges: Vec<Vec<FlowEdge<N, W>>>,
    back_edges: Vec<Vec<FlowEdge<N, W>>>,
    nb_edges: usize,
    nb_vertices: usize,
    in_degree: Vec<usize>,
}
impl<N: Index, W: Weight> Default for FlowNetwork<N, W> {
    fn default() -> Self {
        Self::new()
    }
}
impl<N: Index, W: Weight> FlowNetwork<N, W> {
    /// Creates a new empty graph.
    /// ```
    /// use algods::graph::FlowNetwork;
    /// let graph = FlowNetwork::<u8, u16>::new();
    /// assert_eq!(graph.nb_vertices(), 0);
    /// assert_eq!(graph.nb_edges(), 0);
    /// ```
    pub fn new() -> Self {
        Self {
            out_edges: Vec::new(),
            back_edges: Vec::new(),
            nb_edges: 0,
            nb_vertices: 0,
            in_degree: Vec::new(),
        }
    }
    /// Creates a graph with a given number of vertices and without edges.
    /// ```
    /// use algods::graph::FlowNetwork;
    /// let graph = FlowNetwork::<u16, usize>::init(10);
    /// assert_eq!(graph.nb_vertices(), 10);
    /// assert_eq!(graph.nb_edges(), 0);
    /// ```
    pub fn init(nb_vertices: usize) -> Self {
        assert!(nb_vertices < N::maximum().to_usize());
        let mut graph = Self::new();
        graph.out_edges = vec![Vec::new(); nb_vertices];
        graph.back_edges = vec![Vec::new(); nb_vertices];
        graph.nb_vertices = nb_vertices;
        graph.in_degree = vec![0; nb_vertices];
        graph
    }
    /// Returns the number of edges in the graph.
    /// ```
    /// use algods::graph::FlowNetwork;
    /// let mut graph = FlowNetwork::<u8, u8>::new();
    /// graph.add_vertices(3);
    /// graph.add_edge(0, 2, 0, 4);
    /// graph.add_edge(1, 2, 3, 6);
    /// graph.add_edge(2, 0, 1, 8);
    /// assert_eq!(graph.nb_edges(), 3);
    /// ```
    pub fn nb_edges(&self) -> usize {
        // run time complexity O(1)
        self.nb_edges
    }
    /// Returns the number of vertices in the graph.
    /// ```
    /// use algods::graph::FlowNetwork;
    /// let mut graph = FlowNetwork::<usize, usize>::init(4);
    /// graph.add_edge(0, 2, 8, 10);
    /// graph.add_edge(2, 2, 7, 11);
    /// assert_eq!(graph.nb_vertices(), 4);
    /// ```
    pub fn nb_vertices(&self) -> usize {
        // run time complexity O(1)
        self.nb_vertices
    }
    /// Adds a new edge to the graph
    /// ```
    /// use algods::graph::FlowNetwork;
    /// let mut graph = FlowNetwork::<u8, u16>::init(4);
    /// graph.add_edge(1, 3, 0, 2);
    /// graph.add_edge(1, 2, 2, 4);
    /// graph.add_edge(2, 2, 2, 5);
    /// graph.add_edge(0, 3, 0, 7);
    /// graph.add_edge(1, 2, 2, 4);
    /// graph.add_edge(1, 2, 18, 20);
    /// graph.add_edge(0, 2, 20, 40);
    /// // Remark that there 2 different edges between 1 and 2
    /// assert_eq!(graph.nb_edges(), 6);
    /// assert_eq!(graph.nb_vertices(), 4);
    /// ```
    pub fn add_edge(&mut self, from: N, to: N, flow: W, cap: W) {
        // adds an edge from v to w to the graph
        // run time complexity O(1)
        assert!(flow <= cap);
        assert!(N::to_vertex(self.nb_vertices) >= std::cmp::max(from, to));
        let forward_edge = FlowEdge::init(from, to, flow, cap);
        let backward_edge = FlowEdge::init(to, from, flow, flow);
        if !self.out_edges[from.to_usize()].contains(&forward_edge) {
            println!("len fwd = {}", self.out_edges.len());
            println!("len back = {}", self.back_edges.len());
            self.out_edges[from.to_usize()].push(forward_edge);
            self.back_edges[to.to_usize()].push(backward_edge);
            self.nb_edges += 1;
            self.in_degree[to.to_usize()] += 1;
        }
    }
    /// Adds some vertices to the graph.
    /// ```
    /// use algods::graph::FlowNetwork;
    /// let mut graph = FlowNetwork::<u16, usize>::new();
    /// graph.add_vertices(4);
    /// graph.add_vertex();
    /// graph.add_vertex();
    /// graph.add_edge(0, 1, 2, 2);
    /// graph.add_edge(2, 1, 0, 5);
    /// graph.add_edge(1, 1, 7, 9);
    /// assert_eq!(graph.nb_vertices(), 6);
    /// assert_eq!(graph.nb_edges(), 3);
    /// ```
    pub fn add_vertices(&mut self, nb: usize) {
        let new_size = self.nb_vertices + nb;
        assert!(new_size < N::maximum().to_usize());
        self.out_edges.resize(new_size, Vec::new());
        self.back_edges.resize(new_size, Vec::new());
        self.in_degree.resize(new_size, 0);
        self.nb_vertices += nb;
    }
    /// Adds a vertex to the graph.
    /// ```
    /// use algods::graph::FlowNetwork;
    /// let mut graph = FlowNetwork::<usize, isize>::new();
    /// graph.add_vertex();
    /// graph.add_vertex();
    /// graph.add_vertex();
    /// assert_eq!(graph.nb_vertices(), 3);
    /// ```
    pub fn add_vertex(&mut self) {
        self.out_edges.push(Vec::new());
        self.back_edges.push(Vec::new());
        self.nb_vertices += 1;
    }
    /// Gives a reference to the vertices a given vertex points to.
    /// ```
    /// use algods::graph::{FlowNetwork, FlowEdge};
    /// use std::collections::HashSet;
    /// let mut graph = FlowNetwork::<u32, usize>::new();
    /// graph.add_vertices(3);
    /// graph.add_edge(0, 1, 0, 3);
    /// graph.add_edge(2, 1, 1, 1);
    /// graph.add_edge(0, 2, 3, 4);
    /// let edge_0_1 = FlowEdge::init(0, 1, 0, 3);
    /// let edge_0_2 = FlowEdge::init(0, 2, 3, 4);
    /// assert_eq!(graph.out_edges(&0), &vec![edge_0_1, edge_0_2]);
    /// ```
    pub fn out_edges(&self, vertex: &N) -> &Vec<FlowEdge<N, W>> {
        // gets all the vertices linked to a given vertex v,
        // that is the adjacent vertices of v
        // run time complexity O(1)
        let v = vertex.to_usize();
        &self.out_edges[v]
    }
    pub fn out_edges_mut(&mut self, vertex: &N) -> std::slice::IterMut<'_, FlowEdge<N, W>> {
        // gets all the vertices linked to a given vertex v,
        // that is the adjacent vertices of v
        // run time complexity O(1)
        let v = vertex.to_usize();
        self.out_edges[v].iter_mut()
    }
    pub fn back_edges_mut(&mut self, vertex: &N) -> std::slice::IterMut<'_, FlowEdge<N, W>> {
        // gets all the vertices linked to a given vertex v,
        // that is the adjacent vertices of v
        // run time complexity O(1)
        let v = vertex.to_usize();
        self.back_edges[v].iter_mut()
    }
    /// Returns the edges pointing to a given vertex
    /// ```
    /// use algods::graph::{FlowNetwork, FlowEdge};
    /// use std::collections::HashSet;
    /// let mut graph = FlowNetwork::<u8, u16>::new();
    /// graph.add_vertices(3);
    /// graph.add_edge(2, 1, 0, 0);
    /// graph.add_edge(0, 0, 0, 4);
    /// graph.add_edge(1, 0, 0, 5);
    /// graph.add_edge(0, 2, 0, 10);
    /// graph.add_edge(2, 0, 0, 7);
    /// let edge_0_0 = FlowEdge::init(0, 0, 0, 4);
    /// let edge_1_0 = FlowEdge::init(1, 0, 0, 5);
    /// let edge_2_0 = FlowEdge::init(2, 0, 0, 7);
    /// assert_eq!(graph.in_edges(&0), HashSet::from([&edge_0_0, &edge_1_0, &edge_2_0]));
    /// ```
    pub fn in_edges(&self, vertex: &N) -> HashSet<&FlowEdge<N, W>> {
        self.out_edges
            .iter()
            .filter_map(|adj| adj.iter().find(|&edge| edge.to() == vertex))
            .collect::<HashSet<_>>()
    }
    /// Gives the number of vertices a vertex points to.
    /// ```
    /// use algods::graph::FlowNetwork;
    /// let mut graph = FlowNetwork::<u32, u8>::new();
    /// graph.add_vertices(3);
    /// graph.add_edge(0, 1, 0, 0);
    /// graph.add_edge(2, 1, 0, 0);
    /// graph.add_edge(0, 2, 0, 0);
    /// graph.add_edge(2, 2, 0, 3);
    /// assert_eq!(graph.out_degree(&0), 2);
    /// assert_eq!(graph.out_degree(&1), 0);
    /// assert_eq!(graph.out_degree(&2), 2);
    /// ```
    pub fn out_degree(&self, vertex: &N) -> usize {
        // the number of vertices the vertex v points to
        let v = vertex.to_usize();
        self.out_edges[v].len()
    }
    /// Gives the number of vertices pointing to a vertex
    /// ```
    /// use algods::graph::FlowNetwork;
    /// let mut graph = FlowNetwork::<u8, u8>::new();
    /// graph.add_vertices(3);
    /// graph.add_edge(2, 1, 0, 3);
    /// graph.add_edge(0, 0, 0, 5);
    /// graph.add_edge(1, 0, 0, 8);
    /// graph.add_edge(0, 2, 0, 4);
    /// graph.add_edge(2, 0, 0, 9);
    /// assert_eq!(graph.in_degree(&0), 3);
    /// assert_eq!(graph.in_degree(&1), 1);
    /// assert_eq!(graph.in_degree(&2), 1);
    /// ```
    pub fn in_degree(&self, vertex: &N) -> usize {
        // gives the number of vertices pointing to vertex v
        let v = vertex.to_usize();
        self.in_degree[v]
    }
    /// Gives the integer part of the average number of edges per vertex
    /// # Panics
    /// It panics when there is no vertex in the graph.
    /// ```
    /// use algods::graph::FlowNetwork;
    /// let mut graph = FlowNetwork::<u32, usize>::new();
    /// graph.add_vertices(4);
    /// graph.add_edge(0, 1, 0, 1);
    /// graph.add_edge(0, 0, 10, 10);
    /// graph.add_edge(2, 1, 4, 8);
    /// graph.add_edge(0, 3, 6, 6);
    /// graph.add_edge(1, 3, 4, 4);
    /// graph.add_edge(3, 3, 5, 9);
    /// graph.add_edge(2, 0, 0, 0);
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
    /// use algods::graph::FlowNetwork;
    /// let mut graph = FlowNetwork::<usize, usize>::new();
    /// graph.add_vertices(3);
    /// graph.add_edge(0, 1, 0, 0);
    /// graph.add_edge(0, 0, 1, 3);
    /// graph.add_edge(1, 1, 2, 5);
    /// graph.add_edge(2, 2, 3, 3);
    /// graph.add_edge(2, 1, 4, 5);
    /// graph.add_edge(2, 0, 7, 10);
    /// assert_eq!(graph.self_loop_number(), 3);
    /// ```
    pub fn self_loop_number(&self) -> usize {
        self.out_edges
            .iter()
            .map(|adj| usize::from(adj.iter().any(|edge| edge.from() == edge.to())))
            .sum()
    }
}
impl<N: Index, W: Weight> VertexInfo<N> for FlowNetwork<N, W> {
    fn vertex_edges(&self, vertex: &N) -> Vec<&N> {
        // gets all the vertices linked to a given vertex v,
        // that is the adjacent vertices of v
        let v = vertex.to_usize();
        self.out_edges[v]
            .iter()
            .map(|edge| edge.to())
            .collect::<Vec<&N>>()
    }
    fn nb_vertices(&self) -> usize {
        // run time complexity O(1)
        self.nb_vertices
    }
}
