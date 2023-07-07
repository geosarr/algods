use crate::graph::processing::dfs;
use crate::graph::{Index, VertexInfo};
use std::iter::Rev;
use std::marker::PhantomData;

/// Implementation of the topological sorting algorithm.
pub struct TopologicalSort<N, G>
where
    N: Index,
    G: VertexInfo<N>,
{
    // Sorts vertices of a (edge weighted) directed **acyclic** graph
    // Gives the vertex in reverse order after processing
    reverse_postorder: Vec<N>,
    // Indicates wether or not a vertex w in the graph is visited
    marked: Vec<bool>,
    // Type of the graph
    graph_type: PhantomData<G>,
}
impl<N: Index, G: VertexInfo<N>> TopologicalSort<N, G> {
    /// Creates an empty topological sort structure
    /// ```
    /// use algods::graph::{DiGraph, processing::TopologicalSort};
    /// let topo = TopologicalSort::<u8, DiGraph<u8>>::init(100);
    /// assert_eq!(topo.reverse_postorder(), &vec![]);
    /// ```
    pub fn init(nb_vertices: usize) -> Self {
        Self {
            reverse_postorder: Vec::new(),
            marked: vec![false; nb_vertices],
            graph_type: PhantomData,
        }
    }
    pub fn reverse_postorder(&self) -> &Vec<N> {
        &self.reverse_postorder
    }
    pub fn order(&self) -> Rev<std::slice::Iter<'_, N>> {
        self.reverse_postorder.iter().rev()
    }
    /// Creates an empty topological sort structure
    /// ```
    /// use algods::graph::{DiGraph, processing::TopologicalSort};
    /// let mut graph = DiGraph::<u8>::from_vec(vec![(1, 0), (0, 2), (3, 4), (2, 3)]);
    /// let mut topo = TopologicalSort::init(graph.nb_vertices());
    /// topo.depth_first_order(&graph);
    /// assert_eq!(topo.reverse_postorder(), &vec![4, 3, 2, 0, 1]);
    /// ```
    pub fn depth_first_order(&mut self, graph: &G) {
        let nb = VertexInfo::nb_vertices(graph);
        for v in 0..nb {
            if !self.marked[v] {
                let vertex = N::to_vertex(v);
                dfs(
                    graph,
                    &mut self.marked,
                    &mut self.reverse_postorder,
                    vertex,
                    vertex,
                    false,
                    false,
                );
            }
        }
    }
}
