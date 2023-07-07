#[cfg(test)]
mod unit_test;
use super::Index;
use crate::graph::VertexInfo;
use std::cmp::max;
use std::collections::HashSet;

/// Undirected Graph with an adjacency list structure.
/// ```
/// use algods::graph::Graph;
/// let mut graph = Graph::<u32>::new();
/// graph.add_vertices(4);
/// graph.add_edge(0, 1);
/// graph.add_edge(0, 0);
/// graph.add_edge(1, 2);
/// graph.add_edge(2, 1);
/// assert_eq!(graph.self_loop_number(), 1);
/// assert_eq!(graph.nb_edges(), 3);
/// assert_eq!(graph.nb_vertices(), 4);
/// ```
pub struct Graph<N>
where
    N: Index,
{
    // implements an adjacency-list graph
    // where vertices have indices 0, ..., nb_objects
    // and each vertex is associated to its adjacent vertices
    data: Vec<HashSet<N>>,
    nb_edges: usize,
    nb_vertices: usize,
}
impl<N: Index> Default for Graph<N> {
    fn default() -> Self {
        Self::new()
    }
}
impl<N: Index> Graph<N> {
    /// Creates an empty graph.
    /// ```
    /// use algods::graph::Graph;
    /// let graph = Graph::<u128>::new();
    /// assert_eq!(graph.nb_vertices(), 0);
    /// assert_eq!(graph.nb_edges(), 0);
    /// ```
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            nb_edges: 0,
            nb_vertices: 0,
        }
    }

    /// Creates a graph with a given number of vertices and without edges.
    /// ```
    /// use algods::graph::Graph;
    /// let graph = Graph::<u128>::init(10);
    /// assert_eq!(graph.nb_vertices(), 10);
    /// assert_eq!(graph.nb_edges(), 0);
    /// ```
    pub fn init(nb_vertices: usize) -> Self {
        assert!(nb_vertices < N::maximum().to_usize());
        let mut graph = Self::new();
        graph.nb_vertices = nb_vertices;
        graph.data = vec![HashSet::new(); nb_vertices];
        graph.nb_vertices = nb_vertices;
        graph
    }
    /// Creates a new graph from a `Vec` of edges.
    /// ```
    /// use algods::graph::Graph;
    /// let mut graph = Graph::<u8>::from_vec(vec![(0, 0), (0, 1), (1, 0), (0, 2), (3, 1), (2, 3)]);
    /// assert_eq!(graph.nb_vertices(), 4);
    /// assert_eq!(graph.nb_edges(), 5);
    /// ```
    pub fn from_vec(edges: Vec<(N, N)>) -> Self {
        let mut graph = Self::new();
        let nb_edges = edges.len();
        for edge in edges.iter().take(nb_edges) {
            let vertex_v = edge.0;
            let vertex_w = edge.1;
            let max_vertex = max(vertex_v, vertex_w).to_usize();
            if max_vertex >= graph.nb_vertices {
                graph.add_vertices(max_vertex - graph.nb_vertices + 1);
            }
            graph.add_edge(vertex_v, vertex_w);
        }
        graph
    }
    /// Returns the number of edges in the graph.
    /// ```
    /// use algods::graph::Graph;
    /// let mut graph = Graph::<u8>::new();
    /// graph.add_vertices(3);
    /// graph.add_edge(0, 2);
    /// graph.add_edge(1, 2);
    /// graph.add_edge(2, 0);
    /// assert_eq!(graph.nb_edges(), 2);
    /// ```
    pub fn nb_edges(&self) -> usize {
        // run time complexity O(1)
        self.nb_edges
    }
    /// Returns the number of vertices in the graph.
    /// ```
    /// use algods::graph::Graph;
    /// let mut graph = Graph::<usize>::init(4);
    /// graph.add_edge(0, 2);
    /// graph.add_edge(2, 2);
    /// assert_eq!(graph.nb_vertices(), 4);
    /// ```
    pub fn nb_vertices(&self) -> usize {
        // run time complexity O(1)
        self.nb_vertices
    }
    /// Adds an edge to the graph.
    /// ```
    /// use algods::graph::Graph;
    /// let mut graph = Graph::<u8>::init(4);
    /// graph.add_edge(1, 3);
    /// assert_eq!(graph.nb_edges(), 1);
    /// ```
    pub fn add_edge(&mut self, vertex_v: N, vertex_w: N) {
        // adds an edge to the graph
        // run time complexity O(1)
        let v = vertex_v.to_usize();
        let w = vertex_w.to_usize();
        assert!(self.nb_vertices >= max(v, w));
        let w_is_new: bool = self.data[v].insert(vertex_w);
        let v_is_new: bool = self.data[w].insert(vertex_v);
        self.nb_edges += usize::from(v_is_new || w_is_new);
    }
    /// Adds a vertex to the graph.
    /// ```
    /// use algods::graph::Graph;
    /// let mut graph = Graph::<u8>::new();
    /// graph.add_vertex();
    /// graph.add_vertex();
    /// assert_eq!(graph.nb_vertices(), 2);
    /// ```
    pub fn add_vertex(&mut self) {
        self.data.push(HashSet::new());
        self.nb_vertices += 1;
    }
    /// Adds some vertices to the graph.
    /// ```
    /// use algods::graph::Graph;
    /// let mut graph = Graph::<u16>::new();
    /// graph.add_vertices(4);
    /// graph.add_vertex();
    /// assert_eq!(graph.nb_vertices(), 5);
    /// ```
    pub fn add_vertices(&mut self, nb: usize) {
        let new_size = self.nb_vertices + nb;
        assert!(new_size < N::maximum().to_usize());
        self.data.resize(new_size, HashSet::new());
        self.nb_vertices += nb;
    }
    /// Gives a number of neighbors (vertices) of a given vertex.
    /// ```
    /// use algods::graph::Graph;
    /// let mut graph = Graph::<usize>::new();
    /// graph.add_vertices(3);
    /// graph.add_edge(0, 1);
    /// graph.add_edge(0, 0);
    /// graph.add_edge(2, 0);
    /// assert_eq!(graph.degree(&0), 2);
    /// assert_eq!(graph.degree(&1), 1);
    /// ```
    pub fn degree(&self, vertex: &N) -> usize {
        let v = vertex.to_usize();
        assert!(v < self.nb_vertices);
        self.data[v].len() - usize::from(self.data[v].contains(vertex))
    }
    /// Gives the floor average number of degrees in the graph.
    /// # Panic
    /// It panics when there is no vertex in the graph.
    /// ```
    /// use algods::graph::Graph;
    /// let mut graph = Graph::<u32>::new();
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
        // each edge is counted only once (by the self.add_edge() method)
        if self.nb_vertices > 0 {
            self.nb_edges / self.nb_vertices
        } else {
            panic!("No vertex in the graph");
        }
    }
    /// Returns the number of self loop edges. That is the number of edges linking a vertex to itself.
    /// ```
    /// use algods::graph::Graph;
    /// let mut graph = Graph::<u8>::new();
    /// graph.add_vertices(2);
    /// graph.add_edge(0, 1);
    /// graph.add_edge(0, 0);
    /// graph.add_edge(1, 1);
    /// assert_eq!(graph.self_loop_number(), 2);
    /// ```
    pub fn self_loop_number(&self) -> usize {
        self.data
            .iter()
            .enumerate()
            .map(|(v, e)| usize::from(e.contains(&N::to_vertex(v))))
            .sum()
    }
}
impl<N: Index> VertexInfo<N> for Graph<N> {
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
