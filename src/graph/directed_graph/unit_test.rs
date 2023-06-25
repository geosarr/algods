#[cfg(test)]
mod tests {
    use super::super::{DirectedGraph, EdgeWeightedDigraph};
    use crate::graph::processing::{
        BreadthFirstSearch, DepthFirstSearch, StrongConnectedComponent, TopologicalSort,
    };

    #[test]
    fn test_directed_graph() {
        let n: usize = 10;
        let mut graph = DirectedGraph::init(n);
        assert_eq!(graph.nb_vertices(), n);
        graph.add_edge(0, 5);
        graph.add_edge(4, 8);
        graph.add_edge(7, 4);
        assert_eq!(graph.nb_edges(), 3);
        assert_eq!(graph.out_degree(&2), 0);
        assert_eq!(graph.out_degree(&4), 1);
        assert_eq!(graph.in_degree(&0), 0);
        assert_eq!(graph.in_degree(&8), 1);
        assert_eq!(graph.self_loop_number(), 0);
        graph.add_edge(0, 0);
        assert_eq!(graph.self_loop_number(), 1);
    }

    #[test]
    fn test_edge_weighted_directed_graph() {
        let n: usize = 10;
        let mut graph = EdgeWeightedDigraph::<i8>::init(n);
        assert_eq!(graph.nb_vertices(), n);
        graph.add_edge(0, 5, 1);
        graph.add_edge(4, 8, 1);
        graph.add_edge(7, 4, 1);
        assert_eq!(graph.nb_edges(), 3);
        assert_eq!(graph.out_degree(&2), 0);
        assert_eq!(graph.out_degree(&4), 1);
        assert_eq!(graph.in_degree(&0), 0);
        assert_eq!(graph.in_degree(&4), 1);
        assert_eq!(graph.self_loop_number(), 0);
        graph.add_edge(0, 0, 1);
        assert_eq!(graph.self_loop_number(), 1);
    }

    #[test]
    #[should_panic]
    fn test_directed_graph_panic1() {
        let n: usize = 2;
        let mut graph = DirectedGraph::init(n);
        graph.add_edge(4, 1);
    }

    #[test]
    #[should_panic]
    fn test_directed_graph_panic2() {
        let graph = DirectedGraph::new();
        graph.average_degree();
    }

    #[test]
    fn test_dfs() {
        let mut graph = DirectedGraph::init(9);
        graph.add_edge(0, 1);
        graph.add_edge(0, 2);
        graph.add_edge(0, 6);
        graph.add_edge(0, 5);
        graph.add_edge(6, 4);
        graph.add_edge(4, 3);
        graph.add_edge(5, 4);
        graph.add_edge(5, 3);

        let mut dfs = DepthFirstSearch::init(graph.nb_vertices, 0);
        dfs.find_paths(&graph);
        assert_eq!(dfs.path_to(2), Some(vec![2, 0]));
        assert!(
            dfs.path_to(4) == Some(vec![4, 6, 0])
                || dfs.path_to(4) == Some(vec![4, 5, 0])
                || dfs.path_to(4) == Some(vec![4, 3, 5, 0])
        );
    }

    #[test]
    fn test_bfs() {
        let mut graph = DirectedGraph::init(9);
        graph.add_edge(0, 1);
        graph.add_edge(0, 2);
        graph.add_edge(0, 6);
        graph.add_edge(0, 5);
        graph.add_edge(6, 4);
        graph.add_edge(4, 3);
        graph.add_edge(5, 4);
        graph.add_edge(5, 3);

        let mut bfs = BreadthFirstSearch::init(graph.nb_vertices, 0);
        bfs.find_paths(&graph);
        assert_eq!(bfs.path_to(2), Some(vec![2, 0]));
        assert!(bfs.path_to(4) == Some(vec![4, 6, 0]) || bfs.path_to(4) == Some(vec![4, 5, 0]));
        assert_eq!(bfs.path_to(5), Some(vec![5, 0]));
    }

    #[test]
    fn test_topological_sort() {
        let mut graph = DirectedGraph::init(7);
        graph.add_edge(0, 1);
        graph.add_edge(0, 2);
        graph.add_edge(0, 5);
        graph.add_edge(1, 4);
        graph.add_edge(3, 2);
        graph.add_edge(3, 4);
        graph.add_edge(3, 5);
        graph.add_edge(3, 6);
        graph.add_edge(5, 2);
        graph.add_edge(6, 4);
        graph.add_edge(6, 0);

        let mut topo = TopologicalSort::init(graph.nb_vertices);
        topo.depth_first_order(&graph);
        assert!(
            topo.reverse_postorder() == &vec![4, 1, 2, 5, 0, 6, 3]
                || topo.reverse_postorder() == &vec![2, 5, 4, 1, 0, 6, 3]
                || topo.reverse_postorder() == &vec![2, 4, 1, 5, 0, 6, 3]
        );
        for (pos, a) in topo.order().enumerate() {
            if pos == 0 {
                assert_eq!(a, &3);
            }
            if pos == 1 {
                assert_eq!(a, &6);
            }
            if pos == 2 {
                assert_eq!(a, &0);
            }
            if pos == 3 {
                assert!(a == &5 || a == &1);
            }
            if pos == 4 {
                assert!(a == &1 || a == &2 || a == &4);
            }
            if pos == 5 {
                assert!(a == &1 || a == &5 || a == &4);
            }
            if pos == 6 {
                assert!(a == &2 || a == &4);
            }
        }
    }

    #[test]
    fn test_strong_connected_components() {
        let mut graph = DirectedGraph::init(13);
        graph.add_edge(0, 1);
        graph.add_edge(0, 5);
        graph.add_edge(2, 0);
        graph.add_edge(2, 3);
        graph.add_edge(3, 2);
        graph.add_edge(3, 5);
        graph.add_edge(4, 2);
        graph.add_edge(4, 3);
        graph.add_edge(5, 4);
        graph.add_edge(6, 0);
        graph.add_edge(6, 4);
        graph.add_edge(6, 8);
        graph.add_edge(6, 9);
        graph.add_edge(7, 6);
        graph.add_edge(7, 9);
        graph.add_edge(8, 6);
        graph.add_edge(9, 10);
        graph.add_edge(9, 11);
        graph.add_edge(10, 12);
        graph.add_edge(11, 4);
        graph.add_edge(11, 12);
        graph.add_edge(12, 9);

        let mut scc = StrongConnectedComponent::init(graph.nb_vertices);
        scc.find_scc(&graph);
        assert!(scc.connected(0, 3).unwrap());
        assert!(scc.connected(0, 2).unwrap());
        assert!(scc.connected(0, 4).unwrap());
        assert!(scc.connected(0, 5).unwrap());
        assert!(!scc.connected(0, 1).unwrap());
        assert!(scc.connected(9, 10).unwrap());
        assert!(scc.connected(9, 11).unwrap());
        assert!(scc.connected(11, 10).unwrap());
        assert!(!scc.connected(9, 8).unwrap());
    }
}
