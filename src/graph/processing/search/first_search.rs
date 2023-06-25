use crate::graph::VertexInfo;
use std::collections::LinkedList;

/// Function that runs the depth-first search algorithm
pub fn dfs<G>(
    graph: &G,
    marked: &mut [bool],
    edge_to: &mut Vec<usize>,
    origin: usize,
    component: usize,
    mut_edge_to: bool, // indicates whether or not not mutate edge_to
    is_component: bool, // indicates whether or not to dfs is launched
                       // for connected component-like algorithms
) where
    G: VertexInfo,
{
    // finds all reachable vertices from origin and adds them to the connected component w
    // run time complexity O(sum of degrees of all reachable vertices from origin)
    assert!(VertexInfo::nb_vertices(graph) >= std::cmp::max(origin, component));
    // mark vertex w as visited
    marked[origin] = true;

    // define how to mutate the edge_to list
    let source = if is_component { component } else { origin };
    // recursively visit all unmarked adjacent vertices to w
    let adjacent_vertices = graph.vertex_edges(&origin);
    if mut_edge_to {
        for u in adjacent_vertices {
            if !marked[*u] {
                dfs(
                    graph,
                    marked,
                    edge_to,
                    *u,
                    component,
                    mut_edge_to,
                    is_component,
                );
                edge_to[*u] = source;
            }
        }
    } else {
        for u in adjacent_vertices {
            if !marked[*u] {
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
pub fn bfs<G>(graph: &G, marked: &mut [bool], edge_to: &mut [usize], w: usize)
where
    G: VertexInfo,
{
    assert!(VertexInfo::nb_vertices(graph) >= w);
    let mut queue = LinkedList::<usize>::new();
    // mark the vertex w as visited and add it to the queue
    queue.push_back(w);
    marked[w] = true;

    while let Some(x) = queue.pop_front() {
        // remove the first vertex in the queue
        // add to the queue all unmarked vertices adjacent to v and mark them
        let adj_x = graph.vertex_edges(&x);
        for u in adj_x {
            if !marked[*u] {
                queue.push_back(*u);
                marked[*u] = true;
                edge_to[*u] = x;
            }
        }
    }
}
