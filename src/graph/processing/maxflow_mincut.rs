#[cfg(test)]
mod unit_test;
use crate::graph::{FlowNetwork, Index, Weight};
use std::cmp::min;
use std::collections::VecDeque;

// Define a struct for the graph
#[derive(Debug)]
pub struct FordFulkerson<W>
where
    W: Weight,
{
    max_flow: Option<W>,
}
impl<W: Weight> Default for FordFulkerson<W> {
    fn default() -> Self {
        Self::new()
    }
}
impl<W: Weight> FordFulkerson<W> {
    pub fn new() -> Self {
        Self { max_flow: None }
    }

    fn has_augmenting_path<N: Index>(
        &self,
        network: &mut FlowNetwork<N, W>,
        source: &N,
        destination: &N,
        edge_to: &mut [Option<N>],
    ) -> bool {
        let zero = W::zero();
        let mut marked = vec![false; network.nb_vertices()];
        let mut queue = VecDeque::new();

        marked[source.to_usize()] = true;
        queue.push_back(*source);

        while let Some(vertex) = queue.pop_front() {
            for edge in network.out_edges(&vertex) {
                let next_vertex = edge.to();
                let n_v = next_vertex.to_usize();
                let next_vertex = *next_vertex;
                if !marked[n_v] && edge.residual_capacity() > zero {
                    marked[n_v] = true;
                    edge_to[n_v] = Some(vertex);
                    if next_vertex == *destination {
                        return true;
                    }
                    queue.push_back(next_vertex);
                }
            }
        }
        false
    }
    pub fn max_flow(&self) -> Option<W> {
        self.max_flow
    }
    pub fn find_flows<N: Index>(
        &mut self,
        network: &mut FlowNetwork<N, W>,
        source: &N,
        destination: &N,
    ) {
        let mut edge_to = vec![None; network.nb_vertices()];
        let mut max_flow = Weight::zero();

        while self.has_augmenting_path(network, source, destination, &mut edge_to) {
            let mut path_flow = W::maximum();

            // Find the bottleneck capacity of the path
            let mut vertex = destination;
            while let Some(ref parent_vertex) = edge_to[vertex.to_usize()] {
                let res_cap = network
                    .out_edges(parent_vertex)
                    .iter()
                    .find(|e| e.to() == vertex)
                    .unwrap()
                    .residual_capacity();
                path_flow = min(path_flow, res_cap);
                vertex = parent_vertex;
            }

            // Update the flow of each edge along the path
            vertex = destination;
            while let Some(ref parent_vertex) = edge_to[vertex.to_usize()] {
                let forward_edge = network
                    .out_edges_mut(parent_vertex)
                    .find(|e| e.to() == vertex)
                    .expect("Failed to get forward edge");
                forward_edge.add_residual_flow_to(vertex, path_flow);
                let backward_edge = network
                    .back_edges_mut(vertex)
                    .find(|e| e.to() == parent_vertex)
                    .expect("Failed to get backward edge");
                backward_edge.add_residual_flow_to(parent_vertex, path_flow);
                vertex = parent_vertex;
            }
            max_flow = max_flow + path_flow;
        }
        self.max_flow = Some(max_flow);
    }
}
