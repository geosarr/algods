use crate::graph::{processing::TopologicalSort, EdgeWeightedDigraph, FlowNetwork, Weight};
use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Eq, PartialEq)]
struct CurrentNode<T>
where
    T: std::hash::Hash,
{
    vertex: usize,
    distance: T,
}

// Taken and adapted from the standard library documentation
// for binary heap
impl<T: Ord + std::hash::Hash> Ord for CurrentNode<T> {
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
impl<T: Ord + std::hash::Hash> PartialOrd for CurrentNode<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Function that computes the shortest paths from a source
/// for edge weighted directed acyclic graph with only
/// positive weights using Dijkstra's algorithm
pub fn dijkstra<T: Weight + std::hash::Hash>(
    graph: &EdgeWeightedDigraph<T>,
    source: usize,
    edge_to: &mut Vec<usize>,
    dist_to: &mut Vec<T>,
) {
    let nb = graph.nb_vertices();
    assert_eq!(edge_to.len(), dist_to.len());
    assert_eq!(nb, edge_to.len());

    let mut priority_queue = BinaryHeap::new();
    dist_to[source] = Weight::zero();
    priority_queue.push(CurrentNode {
        vertex: source,
        distance: Weight::zero(),
    });

    while let Some(CurrentNode { vertex, distance }) = priority_queue.pop() {
        let neighbors = graph.vertex_edges(&vertex);
        for (neighbor, dist) in neighbors {
            let node = CurrentNode {
                vertex: *neighbor,
                distance: distance + *dist,
            };
            if dist_to[*neighbor] > node.distance {
                relax(dist_to, edge_to, vertex, *neighbor, *dist);
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

fn relax<T: Weight + std::hash::Hash>(
    dist_to: &mut [T],
    edge_to: &mut [usize],
    origin: usize,
    destination: usize,
    dist: T,
) {
    dist_to[destination] = dist_to[origin] + dist;
    edge_to[destination] = origin;
}

/// Function that computes the shortest paths from a source
/// for edge weighted directed acyclic graphs with possibly
/// negative and/or positive weights
pub fn shortest_path_ewdag<T: Weight + std::hash::Hash>(
    graph: &EdgeWeightedDigraph<T>,
    source: usize,
    edge_to: &mut Vec<usize>,
    dist_to: &mut Vec<T>,
) {
    let nb = graph.nb_vertices();
    assert_eq!(edge_to.len(), dist_to.len());
    assert_eq!(nb, edge_to.len());

    let mut topo = TopologicalSort::init(nb);
    topo.depth_first_order(graph);
    dist_to[source] = Weight::zero();

    // tells whether or not the source
    // vertex is processed in the topological
    // order
    let mut flag_source = false;
    for vertex in topo.order() {
        if *vertex == source {
            flag_source = true;
        }
        if flag_source {
            let neighbors = graph.vertex_edges(vertex);
            for (neighbor, dist) in neighbors {
                if dist_to[*neighbor] > dist_to[*vertex] + *dist {
                    relax(dist_to, edge_to, *vertex, *neighbor, *dist);
                }
            }
        }
    }
}

/// Function that computes the shortest paths from a source
/// for edge weighted directed graph with negative weights  
/// and without any negative cycle
pub fn bellman_ford<T: Weight + std::hash::Hash>(
    graph: &EdgeWeightedDigraph<T>,
    source: usize,
    edge_to: &mut [usize],
    dist_to: &mut [T],
) {
    dist_to[source] = Weight::zero();
    let nb = graph.nb_edges();
    for vertex in 0..nb {
        let adj_v = graph.vertex_edges(&vertex);
        for (u, w) in adj_v {
            if dist_to[*u] > dist_to[vertex] + *w {
                relax(dist_to, edge_to, vertex, *u, *w);
            }
        }
    }
}
