use crate::graph::{processing::TopologicalSort, HashWeightedDiGraph, Weight};
use crate::graph::{Convert, EdgeInfo, Index, Zero};
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::ops::Add;
#[derive(Eq, PartialEq)]
struct CurrentNode<N, W> {
    vertex: N,
    distance: W,
}

// Taken and adapted from the standard library documentation
// for binary heap
impl<N: Ord, W: Ord> Ord for CurrentNode<N, W> {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on distances.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other
            .distance
            .cmp(&self.distance)
            .then_with(|| self.vertex.cmp(&other.vertex))
    }
}
impl<N: Ord, W: Ord> PartialOrd for CurrentNode<N, W> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Function that computes the shortest paths from a source
/// for edge weighted directed acyclic graph with only
/// positive weights using Dijkstra's algorithm
pub fn dijkstra<N: Index, W: Weight + Ord>(
    graph: &HashWeightedDiGraph<N, W>,
    source: N,
    edge_to: &mut Vec<N>,
    dist_to: &mut Vec<W>,
) {
    let nb = graph.nb_vertices();
    assert_eq!(edge_to.len(), dist_to.len());
    assert_eq!(nb, edge_to.len());

    let mut priority_queue = BinaryHeap::new();
    dist_to[source.to_usize()] = W::zero();
    priority_queue.push(CurrentNode {
        vertex: source,
        distance: W::zero(),
    });

    while let Some(CurrentNode { vertex, distance }) = priority_queue.pop() {
        let neighbors = graph.out_edges(&vertex);
        for edge in neighbors {
            let neighbor = *(edge.to());
            let dist = *(edge.weight());
            let node = CurrentNode {
                vertex: neighbor,
                distance: distance + dist,
            };
            if dist_to[neighbor.to_usize()] > node.distance {
                relax(dist_to, edge_to, vertex, neighbor, dist);
                // Not optimal, should see first whether or not
                // the vertex in node is already in the heap
                // if it is the case then update its distance
                // otherwise push it into the heap.
                {
                    priority_queue.push(node);
                }
            }
        }
    }
}

fn relax<N: Convert, W: Copy + Add<Output = W>>(
    dist_to: &mut [W],
    edge_to: &mut [N],
    origin: N,
    destination: N,
    dist: W,
) {
    dist_to[destination.to_usize()] = dist_to[origin.to_usize()] + dist;
    edge_to[destination.to_usize()] = origin;
}

/// Function that computes the shortest paths from a source
/// for edge weighted directed acyclic graphs with possibly
/// negative and/or positive weights
pub fn shortest_path_ewdag<N: Index, W: Weight>(
    graph: &HashWeightedDiGraph<N, W>,
    source: N,
    edge_to: &mut Vec<N>,
    dist_to: &mut Vec<W>,
) {
    let nb = graph.nb_vertices();
    assert_eq!(edge_to.len(), dist_to.len());
    assert_eq!(nb, edge_to.len());

    let mut topo = TopologicalSort::init(nb);
    topo.depth_first_order(graph);
    dist_to[source.to_usize()] = W::zero();

    // tells whether or not the source
    // vertex is processed in the topological
    // order
    let mut flag_source = false;
    for vertex in topo.order() {
        if *vertex == source {
            flag_source = true;
        }
        if flag_source {
            let neighbors = graph.out_edges(vertex);
            for edge in neighbors {
                let neighbor = *(edge.to());
                let dist = *(edge.weight());
                if dist_to[neighbor.to_usize()] > dist_to[vertex.to_usize()] + dist {
                    relax(dist_to, edge_to, *vertex, neighbor, dist);
                }
            }
        }
    }
}

/// Function that computes the shortest paths from a source
/// for edge weighted directed graph with negative weights
/// and without any negative cycle
pub fn bellman_ford<N, W, G>(graph: &G, source: N, edge_to: &mut [N], dist_to: &mut [W])
where
    N: Index,
    W: Copy + Add<Output = W> + Zero + PartialOrd,
    G: EdgeInfo<N, W>,
{
    dist_to[source.to_usize()] = W::zero();
    let nb = graph.nb_edges();
    for v in 0..nb {
        let vertex = N::to_vertex(v);
        let adj_v = graph.out_edges(&vertex);
        for edge in adj_v {
            let u = *(edge.0);
            let w = *(edge.1);
            if dist_to[u.to_usize()] > dist_to[v] + w {
                relax(dist_to, edge_to, vertex, u, w);
            }
        }
    }
}
