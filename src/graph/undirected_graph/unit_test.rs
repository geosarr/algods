#[cfg(test)]
mod tests {
    use super::super::UndirectedGraph;
    use crate::graph::processing::{BreadthFirstSearch, ConnectedComponent, DepthFirstSearch};

    #[test]
    fn test_undirected_graph() {
        let n: usize = 10;
        let mut graph = UndirectedGraph::init(n);
        assert_eq!(graph.nb_vertices(), n);
        graph.add_edge(0, 5);
        graph.add_edge(4, 8);
        graph.add_edge(7, 4);
        assert_eq!(graph.nb_edges(), 3);
        assert_eq!(graph.degree(&1), 0);
        assert_eq!(graph.degree(&4), 2);
        assert_eq!(graph.self_loop_number(), 0);
        graph.add_edge(0, 0);
        assert_eq!(graph.self_loop_number(), 1);
    }

    #[test]
    #[should_panic]
    fn test_undirected_graph_panic1() {
        let n: usize = 2;
        let mut graph = UndirectedGraph::init(n);
        graph.add_edge(4, 1);
    }

    #[test]
    #[should_panic]
    fn test_undirected_graph_panic2() {
        let graph = UndirectedGraph::new();
        graph.average_degree();
    }

    #[test]
    fn test_dfs() {
        let mut graph = UndirectedGraph::init(9);
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
        let mut graph = UndirectedGraph::init(9);
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
    fn test_connected_components() {
        let mut graph = UndirectedGraph::init(13);
        graph.add_edge(0, 1);
        graph.add_edge(0, 2);
        graph.add_edge(0, 6);
        graph.add_edge(0, 5);
        graph.add_edge(6, 4);
        graph.add_edge(4, 3);
        graph.add_edge(5, 4);
        graph.add_edge(5, 3);
        graph.add_edge(7, 8);
        graph.add_edge(9, 10);
        graph.add_edge(9, 11);
        graph.add_edge(9, 12);
        graph.add_edge(11, 12);

        let mut cc = ConnectedComponent::init(graph.nb_vertices);
        cc.find_cc(&graph);
        assert_eq!(cc.count(), 3);
        assert!(cc.connected(0, 3).unwrap());
        assert!(!cc.connected(5, 9).unwrap());
    }
}
