use crate::graph::processing::search::dfs;
// use crate::graph::processing::TopologicalSort;
use crate::graph::{DiGraph, Graph, Index};

use super::TopologicalSort;

/// Implements weakly connected components for an undirected graph.
pub struct ConnectedComponent<N>
where
    N: Index,
{
    // Aims at answering the question are two vertives v and w connected in contant time
    // after preprocessing the graph

    // Identifier of the connected commponent vertices belong to
    id: Vec<N>,
    // Indicates wether or not a vertex w in the graph is visited
    marked: Vec<bool>,
    // Number of connected components
    nb_cc: usize,
    // Whether or not the algorithm has run
    ran: bool,
}
impl<N: Index> ConnectedComponent<N> {
    /// Creates an empty connected component structure
    /// ```
    /// use algods::graph::processing::ConnectedComponent;
    /// let connected_component = ConnectedComponent::<u8>::init(4);
    /// assert_eq!(connected_component.count(), 0);
    /// ```
    pub fn init(nb_vertices: usize) -> Self {
        Self {
            marked: vec![false; nb_vertices],
            id: (0..nb_vertices)
                .map(|v| N::to_vertex(v))
                .collect::<Vec<N>>(),
            nb_cc: 0,
            ran: false,
        }
    }
    /// Finds all the weakly connected components in a graph.
    /// ```
    /// use algods::graph::processing::ConnectedComponent;
    /// use algods::graph::Graph;
    /// let graph = Graph::<u8>::from_vec(vec![(0, 0), (0, 1), (2, 3)]);
    /// let mut connected_component = ConnectedComponent::init(graph.nb_vertices());
    /// connected_component.find(&graph);
    /// assert_eq!(connected_component.count(), 2);
    /// assert!(!connected_component.connected(&1, &2).unwrap());
    /// assert!(connected_component.connected(&0, &1).unwrap());
    /// ```
    pub fn find(&mut self, graph: &Graph<N>) {
        // builds all the connected components from a graph
        let nb = graph.nb_vertices();
        for v in 0..nb {
            if !self.marked[v] {
                // run DFS for each vertex in each component
                let vertex = N::to_vertex(v);
                dfs(
                    graph,
                    &mut self.marked,
                    &mut self.id,
                    vertex,
                    vertex,
                    true,
                    true,
                );
                // here the connected component v is built
                self.nb_cc += 1;
            }
        }
        self.ran = true;
    }
    /// Tests whether or not two nodes are connected.
    /// ```
    /// use algods::graph::processing::ConnectedComponent;
    /// use algods::graph::Graph;
    /// let mut graph = Graph::<u8>::from_vec(vec![(0, 0), (0, 1), (2, 3), (3, 4), (5, 1)]);
    /// graph.add_vertex();
    /// assert_eq!(graph.nb_vertices(), 7);
    /// let mut connected_component = ConnectedComponent::init(graph.nb_vertices());
    /// connected_component.find(&graph);
    /// assert_eq!(connected_component.count(), 3);
    /// assert!(!connected_component.connected(&1, &2).unwrap());
    /// assert!(!connected_component.connected(&6, &3).unwrap());
    /// ```
    pub fn connected(&self, vertex_v: &N, vertex_w: &N) -> Option<bool> {
        // finds out whether or not two vertices are connected
        // run time complexity O(1)
        let v = vertex_v.to_usize();
        let w = vertex_w.to_usize();
        if !self.marked[v] || !self.marked[w] {
            return None;
        }
        Some(self.id[v] == self.id[w])
    }
    /// Gives the number of connected components
    /// ```
    /// use algods::graph::processing::ConnectedComponent;
    /// use algods::graph::Graph;
    /// let mut graph = Graph::<u8>::from_vec(vec![(0, 0), (0, 1), (2, 2), (2, 3), (5, 4)]);
    /// let mut connected_component = ConnectedComponent::init(graph.nb_vertices());
    /// connected_component.find(&graph);
    /// assert_eq!(connected_component.count(), 3);
    /// ```
    pub fn count(&self) -> usize {
        self.nb_cc
    }
    // pub fn is_bipartite(&self) -> Option<bool> {
    //     if self.ran {
    //         Some(self.nb_cc == 1)
    //     } else {
    //         None
    //     }
    // }
}

/// Implements weakly connected components for an undirected graph.
pub struct StrongConnectedComponent<N>
where
    N: Index,
{
    // Aims at answering the question are two vertives v and w connected in contant time
    // after preprocessing a directed graph
    // Identifier of the strong connected commponents vertices belong to
    id: Vec<N>,
    // Indicates wether or not a vertex w in the graph is visited
    marked: Vec<bool>,
    // Number of strong connected components
    nb_scc: usize,
    ran: bool,
}
impl<N: Index> StrongConnectedComponent<N> {
    /// Creates an empty connected component structure
    /// ```
    /// use algods::graph::processing::StrongConnectedComponent;
    /// let strong_connected_component = StrongConnectedComponent::<u8>::init(4);
    /// assert_eq!(strong_connected_component.count(), 0);
    /// ```
    pub fn init(nb_vertices: usize) -> Self {
        Self {
            id: (0..nb_vertices)
                .map(|v| N::to_vertex(v))
                .collect::<Vec<N>>(),
            marked: vec![false; nb_vertices],
            nb_scc: 0,
            ran: false,
        }
    }
    /// Finds all the weakly connected components in a graph.
    /// ```
    /// use algods::graph::processing::StrongConnectedComponent;
    /// use algods::graph::DiGraph;
    /// let graph = DiGraph::<u8>::from_vec(vec![(0, 0), (0, 1), (1, 0), (1, 3), (2, 4), (3, 0), (5, 7)]);
    /// let mut connected_component = StrongConnectedComponent::init(graph.nb_vertices());
    /// connected_component.find(&graph);
    /// assert_eq!(connected_component.count(), 6);
    /// assert!(!connected_component.connected(&1, &2).unwrap());
    /// assert!(connected_component.connected(&0, &1).unwrap());
    /// assert!(!connected_component.connected(&5, &7).unwrap());
    /// assert!(!connected_component.connected(&2, &4).unwrap());
    /// ```
    pub fn find(&mut self, graph: &DiGraph<N>) {
        // builds all the string connected components from a directed graph

        // run dfs on the reverse graph
        let nb = graph.nb_vertices();
        let mut topo = TopologicalSort::init(nb);
        topo.depth_first_order(&graph.reverse());
        let order_second_dfs = topo.reverse_postorder();
        for v in 0..nb {
            let vertex = order_second_dfs[nb - 1 - v];
            if !self.marked[vertex.to_usize()] {
                // run DFS for each vertex in each component
                dfs(
                    graph,
                    &mut self.marked,
                    &mut self.id,
                    vertex,
                    vertex,
                    true,
                    true,
                );
                self.nb_scc += 1;
            }
        }
        self.ran = true;
    }
    /// Tests whether or not two nodes are connected.
    /// ```
    /// use algods::graph::processing::StrongConnectedComponent;
    /// use algods::graph::DiGraph;
    /// let graph = DiGraph::<u8>::from_vec(vec![
    ///     (0, 0),
    ///     (0, 1),
    ///     (1, 0),
    ///     (1, 3),
    ///     (1, 4),
    ///     (2, 4),
    ///     (3, 0),
    ///     (4, 2),
    ///     (5, 7),
    /// ]);
    /// let mut connected_component = StrongConnectedComponent::init(graph.nb_vertices());
    /// connected_component.find(&graph);
    /// assert_eq!(connected_component.count(), 5);
    /// assert!(!connected_component.connected(&1, &2).unwrap());
    /// assert!(connected_component.connected(&0, &1).unwrap());
    /// assert!(!connected_component.connected(&5, &7).unwrap());
    /// assert!(!connected_component.connected(&6, &0).unwrap());
    /// ```
    pub fn connected(&self, vertex_v: &N, vertex_w: &N) -> Option<bool> {
        // finds out whether or not two vertices are connected
        // run time complexity O(1)
        let v = vertex_v.to_usize();
        let w = vertex_w.to_usize();
        if !self.marked[v] || !self.marked[w] {
            return None;
        }
        Some(self.id[v] == self.id[w])
    }
    /// Counts the number of strongly connected components in the graph.
    /// ```
    /// use algods::graph::processing::StrongConnectedComponent;
    /// use algods::graph::DiGraph;
    /// let graph = DiGraph::<u8>::from_vec(vec![(0, 0), (0, 1), (1, 0), (1, 3)]);
    /// assert_eq!(graph.nb_vertices(), 4);
    /// let mut connected_component = StrongConnectedComponent::init(graph.nb_vertices());
    /// connected_component.find(&graph);
    /// assert_eq!(connected_component.count(), 3);
    pub fn count(&self) -> usize {
        self.nb_scc
    }
}
