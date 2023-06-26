#[cfg(test)]
mod unit_test;
use crate::graph::{FlowNetwork, Weight};
use std::cmp::min;
use std::collections::VecDeque;

// Define a struct for the graph
#[derive(Debug)]
pub struct FordFulkerson<T>
where
    T: Weight,
{
    max_flow: Option<T>,
}

impl<T: Weight> FordFulkerson<T> {
    pub fn new() -> Self {
        Self { max_flow: None }
    }

    fn has_augmenting_path(
        &self,
        network: &mut FlowNetwork<T>,
        source: usize,
        destination: usize,
        edge_to: &mut Vec<Option<usize>>,
    ) -> bool {
        let zero = Weight::zero();
        let mut marked = vec![false; network.nb_vertices()];
        let mut queue = VecDeque::new();

        marked[source] = true;
        queue.push_back(source);

        while let Some(vertex) = queue.pop_front() {
            for edge in network.vertex_edges(&vertex) {
                let next_vertex = edge.to();
                let next_vertex = *next_vertex;
                if !marked[next_vertex] && edge.residual_capacity() > zero {
                    marked[next_vertex] = true;
                    edge_to[next_vertex] = Some(vertex);
                    if next_vertex == destination {
                        return true;
                    }
                    queue.push_back(next_vertex);
                }
            }
        }
        false
    }
    pub fn max_flow(&self) -> Option<T> {
        self.max_flow
    }
    fn find_flows(&mut self, network: &mut FlowNetwork<T>, source: usize, destination: usize) {
        let mut edge_to = vec![None; network.nb_vertices()];
        let mut max_flow = Weight::zero();

        while self.has_augmenting_path(network, source, destination, &mut edge_to) {
            let mut path_flow = Weight::max();

            // Find the bottleneck capacity of the path
            let mut vertex = destination;
            while let Some(parent_vertex) = edge_to[vertex] {
                let res_cap = network
                    .vertex_edges(&parent_vertex)
                    .iter()
                    .find(|e| e.to() == &vertex)
                    .unwrap()
                    .residual_capacity();
                path_flow = min(path_flow, res_cap);
                vertex = parent_vertex;
            }

            // Update the flow of each edge along the path
            vertex = destination;
            while let Some(parent_vertex) = edge_to[vertex] {
                let forward_edge = network
                    .vertex_edges_mut(&parent_vertex)
                    .find(|e| e.to() == &vertex)
                    .expect("Failed to get forward edge");
                forward_edge.add_residual_flow_to(&vertex, path_flow);
                let backward_edge = network
                    .vertex_edges_mut(&vertex)
                    .find(|e| e.to() == &parent_vertex)
                    .expect("Failed to get backward edge");
                backward_edge.add_residual_flow_to(&parent_vertex, path_flow);
                vertex = parent_vertex;
            }
            max_flow = max_flow + path_flow;
        }
        self.max_flow = Some(max_flow);
    }
}
