#[cfg(test)]
mod unit_test;
use crate::graph::VertexInfo;
use std::collections::HashSet;

pub struct UndirectedGraph {
    // implements an adjacency-list graph
    // where vertices have indices 0, ..., nb_objects
    // and each vertex is associated to its adjacent vertices
    data: Vec<HashSet<usize>>,
    nb_edges: usize,
    nb_vertices: usize,
}
impl Default for UndirectedGraph {
    fn default() -> Self {
        Self::new()
    }
}
impl UndirectedGraph {
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            nb_edges: 0,
            nb_vertices: 0,
        }
    }

    pub fn init(nb_objects: usize) -> Self {
        let mut graph = Self::new();
        graph.nb_vertices = nb_objects;
        graph.data = Vec::with_capacity(nb_objects);
        for _ in 0..nb_objects {
            graph.data.push(HashSet::<usize>::new());
        }
        graph
    }

    pub fn nb_edges(&self) -> usize {
        // run time complexity O(1)
        self.nb_edges
    }
    pub fn nb_vertices(&self) -> usize {
        // run time complexity O(1)
        self.nb_vertices
    }
    fn vertex_edges(&self, v: &usize) -> &HashSet<usize> {
        // gets all the vertices linked to a given vertex v,
        // that is the adjacent vertices of v
        // run time complexity O(1)
        &self.data[*v]
    }
    pub fn add_edge(&mut self, v: usize, w: usize) {
        // adds an edge to the graph
        // run time complexity O(1)
        assert!(self.nb_vertices >= std::cmp::max(v, w));
        let w_is_in: bool = self.data[v].insert(w);
        let v_is_in: bool = self.data[w].insert(v);
        if v_is_in && w_is_in {
            // v <--> w is a new undirected edge
            self.nb_edges += 1;
        }
    }
    pub fn add_vertex(&mut self) {
        self.data.push(HashSet::<usize>::new());
        self.nb_vertices += 1;
    }

    pub fn degree(&self, v: &usize) -> usize {
        self.vertex_edges(v).len()
    }

    pub fn average_degree(&self) -> usize {
        // gets the average number of degree of the graph
        // each edge is counted only once (by the self.add_edge() method)
        if self.nb_vertices > 0 {
            self.nb_edges / self.nb_vertices
        } else {
            panic!("No vertex in the graph");
        }
    }

    pub fn self_loop_number(&self) -> usize {
        self.data
            .iter()
            .enumerate()
            .map(|(v, e)| usize::from(e.contains(&v)))
            .sum()
    }
}
impl VertexInfo for UndirectedGraph {
    fn vertex_edges(&self, v: &usize) -> Vec<&usize> {
        // gets all the vertices linked to a given vertex v,
        // that is the adjacent vertices of v
        // run time complexity O(1)
        self.data[*v].iter().collect::<Vec<&usize>>()
    }
    fn nb_vertices(&self) -> usize {
        // run time complexity O(1)
        self.nb_vertices
    }
}
