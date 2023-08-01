mod first_search;
mod shortest_path;
#[cfg(test)]
mod unit_test;
use crate::graph::{BaseWeight, EdgeInfo, Index, VertexInfo, Zero};
pub use first_search::{bfs, dfs};
pub use shortest_path::{
    bellman_ford, dijkstra, shortest_path_ewdag, shortest_path_faster_algorithm,
};
use std::ops::Add;

pub struct DepthFirstSearch<N> {
    // Indicates whether or not a vertex w in the graph is visited
    marked: Vec<bool>,
    // Indicates what is the previous vertex leading to the current vertex
    // when edge_to[w]=w, then no path is found yet from v to w
    edge_to: Vec<N>,
    // vertex for which paths are computed
    v: N,
}
impl<N: Index> DepthFirstSearch<N> {
    pub fn init(nb_vertices: usize, origin: N) -> Self {
        Self {
            marked: vec![false; nb_vertices],
            edge_to: (0..nb_vertices)
                .map(|v| N::to_vertex(v))
                .collect::<Vec<N>>(),
            v: origin,
        }
    }

    pub fn find_paths<G>(&mut self, graph: &G)
    where
        G: VertexInfo<N>,
    {
        // finds all paths from self.v in self.graph
        dfs(
            graph,
            &mut self.marked,
            &mut self.edge_to,
            self.v,
            self.v,
            true,
            false,
        );
    }

    pub fn path_to(&self, target: &N) -> Option<Vec<N>> {
        // finds the path from v to w
        // run time complexity O(length of the path)
        // can be very time consuming for some applications
        let t = target.to_usize();
        if !self.marked[t] {
            return None;
        }
        let mut path = Vec::<N>::new();
        let mut x = *target;
        while x != self.v {
            path.push(x);
            x = self.edge_to[x.to_usize()];
        }
        path.push(self.v);
        Some(path)
    }
}

pub struct BreadthFirstSearch<N> {
    // Indicates wether or not a vertex w in the graph is visited
    marked: Vec<bool>,
    // Indicates what is the previous vertex leading to the current vertex
    // when edge_to[w]=w, then no path is found yet from v to w
    edge_to: Vec<N>,
    // Vertex for which paths are computed
    v: N,
}
impl<N: Index> BreadthFirstSearch<N> {
    pub fn init(nb_vertices: usize, origin: N) -> Self {
        Self {
            marked: vec![false; nb_vertices],
            edge_to: (0..nb_vertices)
                .map(|v| N::to_vertex(v))
                .collect::<Vec<N>>(),
            v: origin,
        }
    }

    pub fn find_paths<G>(&mut self, graph: &G)
    where
        G: VertexInfo<N>,
    {
        // finds all reachable vertices from w
        bfs(graph, &mut self.marked, &mut self.edge_to, self.v);
    }

    pub fn path_to(&self, target: &N) -> Option<Vec<N>> {
        // finds the path from v to w
        // run time complexity O(length of the path)
        // computes shortest paths
        let t = target.to_usize();
        if !self.marked[t] {
            return None;
        }
        let mut path = Vec::<N>::new();
        let mut x = *target;
        while x != self.v {
            path.push(x);
            x = self.edge_to[x.to_usize()];
        }
        path.push(self.v);
        Some(path)
    }
}

pub struct ShortestPath<N, W> {
    // the source vertex from where the shortest
    // paths are computed
    source: N,
    // stores the length of the shortest path from
    // the source to an edge
    dist_to: Vec<W>,
    // stores the vertex that is the closest
    // to an edge in the shortest path
    edge_to: Vec<N>,
}
impl<N: Index, W: BaseWeight> ShortestPath<N, W> {
    pub fn init(from: N, nb_vertices: usize) -> Self {
        Self {
            source: from,
            dist_to: vec![W::maximum(); nb_vertices],
            edge_to: vec![N::maximum(); nb_vertices],
        }
    }
}

impl<N: Index, W> ShortestPath<N, W> {
    pub fn dist_to(&self, vertex: &N) -> &W {
        let v = vertex.to_usize();
        &self.dist_to[v]
    }
    pub fn edge_to(&self, vertex: &N) -> &N {
        let v = vertex.to_usize();
        &self.edge_to[v]
    }
}
impl<N: Index, W: BaseWeight> ShortestPath<N, W> {
    pub fn path_to(&self, vertex: &N) -> Option<Vec<N>> {
        let v = vertex.to_usize();
        if self.dist_to[v] == W::maximum() {
            return None;
        }
        let mut path = Vec::new();
        let mut origin = *vertex;
        while origin != self.source {
            path.push(origin);
            origin = self.edge_to[origin.to_usize()];
        }
        path.push(self.source);
        Some(path)
    }
}

impl<N, W> ShortestPath<N, W> {
    pub fn dijkstra<G>(&mut self, graph: &G)
    where
        N: Index,
        W: Copy + Zero + Ord + Add<Output = W>,
        G: EdgeInfo<N, W> + VertexInfo<N>,
    {
        dijkstra(graph, self.source, &mut self.edge_to, &mut self.dist_to);
    }
    pub fn ewdag<G>(&mut self, graph: &G)
    where
        N: Index,
        W: BaseWeight,
        G: VertexInfo<N> + EdgeInfo<N, W>,
    {
        shortest_path_ewdag(graph, self.source, &mut self.edge_to, &mut self.dist_to);
    }
    pub fn bellman_ford<G>(&mut self, graph: &G)
    where
        N: Index,
        W: Copy + Add<Output = W> + Zero + PartialOrd,
        G: EdgeInfo<N, W>,
    {
        bellman_ford(graph, self.source, &mut self.edge_to, &mut self.dist_to);
    }

    pub fn spfa<G>(&mut self, graph: &G)
    where
        N: Index,
        W: Copy + Add<Output = W> + Zero + PartialOrd,
        G: EdgeInfo<N, W>,
    {
        shortest_path_faster_algorithm(graph, self.source, &mut self.edge_to, &mut self.dist_to);
    }
}
