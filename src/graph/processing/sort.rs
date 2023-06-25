use crate::graph::processing::dfs;
use crate::graph::VertexInfo;
use std::iter::Rev;
use std::marker::PhantomData;

pub struct TopologicalSort<G> {
    // Sorts vertices of a (edge weighted) directed **acyclic** graph
    // Gives the vertex in reverse order after processing
    reverse_postorder: Vec<usize>,
    // Indicates wether or not a vertex w in the graph is visited
    marked: Vec<bool>,
    // Type of the graph
    graph_type: PhantomData<G>,
}
impl<G: VertexInfo> TopologicalSort<G> {
    pub fn init(nb_vertices: usize) -> Self {
        Self {
            reverse_postorder: Vec::new(),
            marked: vec![false; nb_vertices],
            graph_type: PhantomData,
        }
    }
    pub fn reverse_postorder(&self) -> &Vec<usize> {
        &self.reverse_postorder
    }
    pub fn order(&self) -> Rev<std::slice::Iter<'_, usize>> {
        self.reverse_postorder.iter().rev()
    }
    pub fn depth_first_order(&mut self, graph: &G) {
        let nb = VertexInfo::nb_vertices(graph);
        for v in 0..nb {
            if !self.marked[v] {
                // run DFS for each vertex in each component
                dfs(
                    graph,
                    &mut self.marked,
                    &mut self.reverse_postorder,
                    v,
                    v,
                    false,
                    false,
                );
            }
        }
    }
}
