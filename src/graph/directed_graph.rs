#[cfg(test)]
mod unit_test;
use crate::graph::{VertexInfo, Weight};
use crate::utils::read_lines;
use std::collections::HashSet;
use std::path::Path;

#[derive(Eq, Hash, PartialEq, Copy, Clone)]
pub struct DirectedEdge {
    from: usize, // not necessarily useful but keeps the idea of an edge
    to: usize,
}

impl DirectedEdge {
    pub fn init(origin: usize, destination: usize) -> Self {
        Self {
            from: origin,
            to: destination,
        }
    }
    pub fn to(&self) -> &usize {
        &self.to
    }
    pub fn from(&self) -> &usize {
        &self.from
    }
}
/// Implementation of an adjacency-list based unweighted directed graph
/// ```
/// use algods::graph::DirectedGraph;
/// let mut graph = DirectedGraph::init(3);
/// graph.add_edge(0,1);
/// graph.add_edge(1,2);
/// assert_eq!(graph.nb_vertices(), 3);
/// assert_eq!(graph.nb_edges(), 2);
/// graph.add_vertex();
/// assert_eq!(graph.nb_vertices(), 4);
/// ```
pub struct DirectedGraph {
    // implements an adjacency-list graph
    // where vertices have indices 0, ..., nb_objects
    // and each vertex is associated to the vertices it points to
    data: Vec<HashSet<DirectedEdge>>,
    nb_edges: usize,
    nb_vertices: usize,
    in_edges: Vec<HashSet<usize>>,
}
impl Default for DirectedGraph {
    fn default() -> Self {
        Self::new()
    }
}
impl DirectedGraph {
    /// Creates a new empty graph.
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            nb_edges: 0,
            nb_vertices: 0,
            in_edges: Vec::new(),
        }
    }
    /// Creates a new graph with unconnected `nb_objects` objects
    pub fn init(nb_objects: usize) -> Self {
        let mut graph = Self::new();
        graph.nb_vertices = nb_objects;
        graph.data = Vec::with_capacity(nb_objects);
        for _ in 0..nb_objects {
            graph.data.push(HashSet::new());
            graph.in_edges.push(HashSet::new());
        }
        graph
    }
    /// Creates a new graph which has the same vertices but edges reverted.
    pub fn reverse(&self) -> Self {
        // Gets the reverse graph
        let nb_vertices = self.nb_vertices;
        let mut rev_graph = Self::init(nb_vertices);
        for v in 0..nb_vertices {
            let adj_v = self.vertex_edges(&v);
            for w in adj_v {
                rev_graph.add_edge(*w, v);
            }
        }
        rev_graph
    }
    /// Creates a graph from a file
    pub fn from_file<P>(filename: P, sep: char, nb_vertices: usize) -> Self
    where
        P: AsRef<Path>,
    {
        // Builds a directed graph from a file with edges.
        // All the elements of each row should be non
        // negative integers separated by the value of the sep
        // argument, each row represent one or many edges from the first vertex to
        // the other ones. If there is only one value, it will be skipped
        let mut nb_iter = 0;
        println!("Initializing the graph");
        let mut dg = DirectedGraph::init(nb_vertices);
        match read_lines(filename) {
            Ok(lines) => {
                for (_, line) in lines.enumerate() {
                    if let Ok(row) = line {
                        let values = row.split(sep).collect::<Vec<&str>>();
                        for i in 1..values.len() {
                            dg.add_edge(
                                values[0].parse::<usize>().unwrap(),
                                values[i].parse::<usize>().unwrap(),
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
        dg
    }

    /// Gives the number of edges
    pub fn nb_edges(&self) -> usize {
        // run time complexity O(1)
        self.nb_edges
    }
    /// Gives the number of vertices
    pub fn nb_vertices(&self) -> usize {
        // run time complexity O(1)
        self.nb_vertices
    }
    /// Adds a new edge o the graph
    pub fn add_edge(&mut self, v: usize, w: usize) {
        // adds an edge from v to w to the graph
        // run time complexity O(1)
        assert!(self.nb_vertices >= std::cmp::max(v, w));
        let edge = DirectedEdge::init(v, w);
        let w_is_in = self.data[v].insert(edge);
        self.in_edges[w].insert(v);
        if w_is_in {
            // v --> w is a new directed edge
            self.nb_edges += 1;
        }
    }
    /// Adds a new vertex to the graph
    pub fn add_vertex(&mut self) {
        self.data.push(HashSet::<DirectedEdge>::new());
        self.nb_vertices += 1;
    }
    /// Returns an immutable reference to the set of edges
    pub fn vertex_edges(&self, v: &usize) -> Vec<&usize> {
        // gets all the vertices linked to a given vertex v,
        // that is the adjacent vertices of v
        // run time complexity O(1)
        self.data[*v]
            .iter()
            .map(|edge| edge.to())
            .collect::<Vec<&usize>>()
    }
    ///
    pub fn out_edges(&self, v: &usize) -> &HashSet<DirectedEdge> {
        // gets all the vertices linked to a given vertex v,
        // that is the adjacent vertices of v
        // run time complexity O(1)
        &self.data[*v]
    }
    ///
    pub fn in_edges(&self, v: &usize) -> &HashSet<usize> {
        // self.data
        //     .iter()
        //     .filter_map(|adj| adj.iter().find(|e| e.to() == v))
        //     .map(|e| e.from())
        //     .collect::<Vec<&usize>>()
        &self.in_edges[*v]
    }
    /// Gives the number of vertices a vertex point to
    pub fn out_degree(&self, v: &usize) -> usize {
        // the number of vertices the vertex v points to
        self.vertex_edges(v).len()
    }
    /// Gives the number of vertices pointing to a vertex
    pub fn in_degree(&self, v: &usize) -> usize {
        // gives the number of vertices pointing to vertex v
        self.data
            .iter()
            .map(|adj| usize::from(adj.iter().any(|e| e.to() == v)))
            .sum()
    }
    /// Gives the integer part of the average number of edges per vertex
    pub fn average_degree(&self) -> usize {
        // gets the average number of degree of the graph
        if self.nb_vertices > 0 {
            self.nb_edges / self.nb_vertices
        } else {
            panic!("No vertex in the graph");
        }
    }
    /// Returns the number of vertices pointing to themselves
    pub fn self_loop_number(&self) -> usize {
        self.data
            .iter()
            .enumerate()
            .map(|(v, e)| usize::from(e.contains(&DirectedEdge::init(v, v))))
            .sum()
    }
}
impl VertexInfo for DirectedGraph {
    fn vertex_edges(&self, v: &usize) -> Vec<&usize> {
        // gets all the vertices linked to a given vertex v,
        // that is the adjacent vertices of v
        // run time complexity O(1)
        self.vertex_edges(v)
    }
    fn nb_vertices(&self) -> usize {
        // run time complexity O(1)
        self.nb_vertices
    }
}

#[derive(Eq, Hash, PartialEq, Copy, Clone, Debug)]
struct DirectedWeightedEdge<T>
where
    T: Weight,
{
    from: usize, // not necessarily useful but keeps the idea of an edge
    to: usize,
    weight: T,
}
impl<T: Weight> DirectedWeightedEdge<T> {
    pub fn init(origin: usize, destination: usize, cost: T) -> Self {
        Self {
            from: origin,
            to: destination,
            weight: cost,
        }
    }
    pub fn to(&self) -> &usize {
        &self.to
    }
    pub fn from(&self) -> &usize {
        &self.from
    }
    pub fn weight(&self) -> &T {
        &self.weight
    }
}

pub struct EdgeWeightedDigraph<T>
where
    T: Weight,
{
    data: Vec<HashSet<DirectedWeightedEdge<T>>>,
    nb_edges: usize,
    nb_vertices: usize,
}

impl<T: Weight> Default for EdgeWeightedDigraph<T> {
    fn default() -> Self {
        Self::new()
    }
}
impl<T: Weight> EdgeWeightedDigraph<T> {
    /// Creates a new empty graph.
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            nb_edges: 0,
            nb_vertices: 0,
        }
    }
    /// Creates a new graph with unconnected `nb_objects` objects
    pub fn init(nb_objects: usize) -> Self {
        let mut graph = Self::new();
        graph.nb_vertices = nb_objects;
        graph.data = Vec::with_capacity(nb_objects);
        for _ in 0..nb_objects {
            graph.data.push(HashSet::new());
        }
        graph
    }
    /// Gives the number of edges
    pub fn nb_edges(&self) -> usize {
        // run time complexity O(1)
        self.nb_edges
    }
    /// Gives the number of vertices
    pub fn nb_vertices(&self) -> usize {
        // run time complexity O(1)
        self.nb_vertices
    }
    /// Adds a new edge of the graph
    pub fn add_edge(&mut self, u: usize, v: usize, w: T) {
        // adds an edge from v to w to the graph
        // run time complexity O(1)
        assert!(self.nb_vertices >= std::cmp::max(u, v));
        let edge = DirectedWeightedEdge::init(u, v, w);
        // println!("{edge:?}");
        let is_new = self.data[u].insert(edge);
        if is_new {
            // u --> v is a new directed edge
            self.nb_edges += 1;
        }
    }
    /// Adds a new vertex to the graph
    pub fn add_vertex(&mut self) {
        self.data.push(HashSet::new());
        self.nb_vertices += 1;
    }
    /// Returns an immutable reference to the set of edges
    pub fn vertex_edges(&self, v: &usize) -> Vec<(&usize, &T)> {
        // gets all the vertices linked to a given vertex v,
        // that is the adjacent vertices of v
        // run time complexity O(1)
        self.data[*v]
            .iter()
            .map(|edge| (edge.to(), edge.weight()))
            .collect::<Vec<(&usize, &T)>>()
    }
    /// Gives the number of vertices a vertex point to
    pub fn out_degree(&self, v: &usize) -> usize {
        // the number of vertices the vertex v points to
        self.vertex_edges(v).len()
    }
    /// Gives the number of vertices pointing to a vertex
    pub fn in_degree(&self, v: &usize) -> usize {
        // gives the number of vertices pointing to vertex v
        self.data
            .iter()
            .map(|adj| usize::from(adj.iter().any(|edge| v == edge.to())))
            .sum()
    }
    /// Gives the integer part of the average number of edges per vertex
    pub fn average_degree(&self) -> usize {
        // gets the average number of degree of the graph
        if self.nb_vertices > 0 {
            self.nb_edges / self.nb_vertices
        } else {
            panic!("No vertex in the graph");
        }
    }
    /// Returns the number of vertices pointing to themselves
    pub fn self_loop_number(&self) -> usize {
        self.data
            .iter()
            .map(|adj| usize::from(adj.iter().any(|edge| edge.from() == edge.to())))
            .sum()
    }
}
impl<T: Weight> VertexInfo for EdgeWeightedDigraph<T> {
    fn vertex_edges(&self, v: &usize) -> Vec<&usize> {
        // gets all the vertices linked to a given vertex v,
        // that is the adjacent vertices of v
        self.data[*v]
            .iter()
            .map(|edge| edge.to())
            .collect::<Vec<&usize>>()
    }
    fn nb_vertices(&self) -> usize {
        // run time complexity O(1)
        self.nb_vertices
    }
}

#[derive(Debug, Eq, Hash, PartialEq, Copy, Clone)]
pub struct FlowEdge<T>
where
    T: Weight,
{
    from: usize,
    to: usize,
    flow: T,
    capacity: T,
}

impl<T: Weight> FlowEdge<T> {
    pub fn init(origin: usize, destination: usize, f: T, c: T) -> Self {
        Self {
            from: origin,
            to: destination,
            flow: f,
            capacity: c,
        }
    }

    pub fn from(&self) -> &usize {
        &self.from
    }

    pub fn to(&self) -> &usize {
        &self.to
    }

    pub fn flow(&self) -> &T {
        &self.flow
    }
    pub fn flow_mut(&mut self) -> &mut T {
        &mut self.flow
    }

    pub fn capacity(&self) -> &T {
        &self.capacity
    }
}

impl<T: Weight> FlowEdge<T> {
    pub fn residual_capacity(&self) -> T {
        self.capacity - self.flow
    }
    pub fn add_residual_flow_to(&mut self, vertex: &usize, delta: T) {
        if vertex == self.from() {
            self.flow = self.flow - delta;
        } else if vertex == self.to() {
            self.flow = self.flow + delta;
        } else {
            panic!("Illegal endpoint {vertex}")
        }
    }
}

pub struct FlowNetwork<T>
where
    T: Weight,
{
    data: Vec<Vec<FlowEdge<T>>>,
    nb_edges: usize,
    nb_vertices: usize,
}

impl<T: Weight> FlowNetwork<T> {
    /// Creates a new empty graph.
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            nb_edges: 0,
            nb_vertices: 0,
        }
    }
    /// Creates a new graph with unconnected `nb_objects` objects
    pub fn init(nb_objects: usize) -> Self {
        let mut graph = Self::new();
        graph.nb_vertices = nb_objects;
        graph.data = Vec::with_capacity(nb_objects);
        for _ in 0..nb_objects {
            graph.data.push(Vec::new());
        }
        graph
    }
    /// Gives the number of edges
    pub fn nb_edges(&self) -> usize {
        // run time complexity O(1)
        self.nb_edges
    }
    /// Gives the number of vertices
    pub fn nb_vertices(&self) -> usize {
        // run time complexity O(1)
        self.nb_vertices
    }
    /// Adds a new edge of the graph
    pub fn add_edge(&mut self, from: usize, to: usize, cap: T) {
        // adds an edge from v to w to the graph
        // run time complexity O(1)
        assert!(self.nb_vertices >= std::cmp::max(from, to));
        let zero = Weight::zero();
        let forward_edge = FlowEdge::init(from, to, zero, cap);
        let backward_edge = FlowEdge::init(to, from, zero, zero);
        if !self.data[from].contains(&forward_edge) {
            self.data[from].push(forward_edge);
            self.data[to].push(backward_edge);
            self.nb_edges += 1;
        }
    }
    /// Adds a new vertex to the graph
    pub fn add_vertex(&mut self) {
        self.data.push(Vec::new());
        self.nb_vertices += 1;
    }
    /// Returns an immutable reference to the set of edges
    pub fn vertex_edges(&self, v: &usize) -> Vec<&FlowEdge<T>> {
        // gets all the vertices linked to a given vertex v,
        // that is the adjacent vertices of v
        // run time complexity O(1)
        self.data[*v].iter().collect::<Vec<&FlowEdge<T>>>()
    }
    pub fn vertex_edges_mut(&mut self, v: &usize) -> std::slice::IterMut<'_, FlowEdge<T>> {
        // gets all the vertices linked to a given vertex v,
        // that is the adjacent vertices of v
        // run time complexity O(1)
        self.data[*v].iter_mut()
    }
    /// Gives the number of vertices a vertex point to
    pub fn out_degree(&self, v: &usize) -> usize {
        // the number of vertices the vertex v points to
        self.vertex_edges(v).len()
    }
    /// Gives the number of vertices pointing to a vertex
    pub fn in_degree(&self, v: &usize) -> usize {
        // gives the number of vertices pointing to vertex v
        self.data
            .iter()
            .map(|adj| usize::from(adj.iter().any(|edge| v == edge.to())))
            .sum()
    }
    /// Gives the integer part of the average number of edges per vertex
    pub fn average_degree(&self) -> usize {
        // gets the average number of degree of the graph
        if self.nb_vertices > 0 {
            self.nb_edges / self.nb_vertices
        } else {
            panic!("No vertex in the graph");
        }
    }
    /// Returns the number of vertices pointing to themselves
    pub fn self_loop_number(&self) -> usize {
        self.data
            .iter()
            .map(|adj| usize::from(adj.iter().any(|edge| edge.from() == edge.to())))
            .sum()
    }
}
