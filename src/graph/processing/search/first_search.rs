use crate::graph::{Index, VertexInfo};
use std::collections::VecDeque;

/// Function that runs the depth-first search algorithm
pub fn dfs<N, G>(
    graph: &G,
    marked: &mut [bool],
    edge_to: &mut Vec<N>,
    origin: N,
    component: N,
    mut_edge_to: bool, // indicates whether or not to mutate edge_to
    is_component: bool, // indicates whether or not dfs is launched
                       // for connected component-like algorithms
) where
    N: Index,
    G: VertexInfo<N>,
{
    // finds all reachable vertices from origin and adds them to the connected component w
    // run time complexity O(sum of degrees of all reachable vertices from origin)
    let orig = origin.to_usize();
    let comp = component.to_usize();
    assert!(VertexInfo::nb_vertices(graph) >= std::cmp::max(orig, comp));
    // mark vertex w as visited
    marked[orig] = true;

    // define how to mutate the edge_to list
    let source = if is_component { component } else { origin };
    // recursively visit all unmarked adjacent vertices to w
    let adjacent_vertices = graph.vertex_edges(&origin);
    if mut_edge_to {
        for u in adjacent_vertices {
            let neighbor = u.to_usize();
            if !marked[neighbor] {
                dfs(
                    graph,
                    marked,
                    edge_to,
                    *u,
                    component,
                    mut_edge_to,
                    is_component,
                );
                edge_to[neighbor] = source;
            }
        }
    } else {
        for u in adjacent_vertices {
            let neighbor = u.to_usize();
            if !marked[neighbor] {
                dfs(
                    graph,
                    marked,
                    edge_to,
                    *u,
                    component,
                    mut_edge_to,
                    is_component,
                );
            }
        }
        edge_to.push(origin);
    }
}

/// Function that runs the breadth-first search algorithm
pub fn bfs<N, G>(graph: &G, marked: &mut [bool], edge_to: &mut [N], vertex_w: N)
where
    N: Index,
    G: VertexInfo<N>,
{
    let w = vertex_w.to_usize();
    assert!(VertexInfo::nb_vertices(graph) >= w);
    let mut queue = VecDeque::<N>::new();
    // mark the vertex w as visited and add it to the queue
    queue.push_back(vertex_w);
    marked[w] = true;

    while let Some(node) = queue.pop_front() {
        // remove the first vertex in the queue
        // add to the queue all unmarked vertices adjacent to v and mark them
        let adj_node = graph.vertex_edges(&node);
        for vertex_v in adj_node {
            let v = vertex_v.to_usize();
            if !marked[v] {
                queue.push_back(*vertex_v);
                marked[v] = true;
                edge_to[v] = node;
            }
        }
    }
}
