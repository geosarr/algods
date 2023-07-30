#[cfg(test)]
mod tests {
    use super::super::ShortestPath;
    use crate::graph::{HashWeightedDiGraph, VecWeightedDiGraph};

    #[test]
    fn test_dijkstra() {
        let mut graph = HashWeightedDiGraph::<u32, u8>::init(9);
        graph.add_edge(0, 1, 1);
        graph.add_edge(0, 2, 1);
        graph.add_edge(0, 6, 2);
        graph.add_edge(0, 5, 3);
        graph.add_edge(0, 7, 5);
        graph.add_edge(4, 3, 1);
        graph.add_edge(4, 5, 4);
        graph.add_edge(5, 3, 2);
        graph.add_edge(6, 7, 2);
        graph.add_edge(6, 4, 1);

        let mut shortest_path = ShortestPath::init(0, graph.nb_vertices());
        shortest_path.dijkstra(&graph);
        // println!("{:?}",graph.vertex_edges(&0));
        assert_eq!(shortest_path.path_to(&0), Some(vec![0]));
        assert_eq!(shortest_path.path_to(&1), Some(vec![1, 0]));
        assert_eq!(shortest_path.path_to(&2), Some(vec![2, 0]));
        assert_eq!(shortest_path.path_to(&3), Some(vec![3, 4, 6, 0]));
        assert_eq!(shortest_path.path_to(&4), Some(vec![4, 6, 0]));
        assert_eq!(shortest_path.path_to(&5), Some(vec![5, 0]));
        assert_eq!(shortest_path.path_to(&6), Some(vec![6, 0]));
        assert_eq!(shortest_path.path_to(&7), Some(vec![7, 6, 0]));
        assert_eq!(shortest_path.path_to(&8), None);

        let mut shortest_path = ShortestPath::init(1, graph.nb_vertices());
        shortest_path.dijkstra(&graph);
        assert_eq!(shortest_path.path_to(&0), None);
        assert_eq!(shortest_path.path_to(&1), Some(vec![1]));
        assert_eq!(shortest_path.path_to(&2), None);
        assert_eq!(shortest_path.path_to(&3), None);
        assert_eq!(shortest_path.path_to(&4), None);
        assert_eq!(shortest_path.path_to(&5), None);
        assert_eq!(shortest_path.path_to(&6), None);
        assert_eq!(shortest_path.path_to(&7), None);
        assert_eq!(shortest_path.path_to(&8), None);

        let mut shortest_path = ShortestPath::init(2, graph.nb_vertices());
        shortest_path.dijkstra(&graph);
        assert_eq!(shortest_path.path_to(&0), None);
        assert_eq!(shortest_path.path_to(&1), None);
        assert_eq!(shortest_path.path_to(&2), Some(vec![2]));
        assert_eq!(shortest_path.path_to(&3), None);
        assert_eq!(shortest_path.path_to(&4), None);
        assert_eq!(shortest_path.path_to(&5), None);
        assert_eq!(shortest_path.path_to(&6), None);
        assert_eq!(shortest_path.path_to(&7), None);
        assert_eq!(shortest_path.path_to(&8), None);

        let mut shortest_path = ShortestPath::init(3, graph.nb_vertices());
        shortest_path.dijkstra(&graph);
        assert_eq!(shortest_path.path_to(&0), None);
        assert_eq!(shortest_path.path_to(&1), None);
        assert_eq!(shortest_path.path_to(&2), None);
        assert_eq!(shortest_path.path_to(&3), Some(vec![3]));
        assert_eq!(shortest_path.path_to(&4), None);
        assert_eq!(shortest_path.path_to(&5), None);
        assert_eq!(shortest_path.path_to(&6), None);
        assert_eq!(shortest_path.path_to(&7), None);
        assert_eq!(shortest_path.path_to(&8), None);

        let mut shortest_path = ShortestPath::init(4, graph.nb_vertices());
        shortest_path.dijkstra(&graph);
        assert_eq!(shortest_path.path_to(&0), None);
        assert_eq!(shortest_path.path_to(&1), None);
        assert_eq!(shortest_path.path_to(&2), None);
        assert_eq!(shortest_path.path_to(&3), Some(vec![3, 4]));
        assert_eq!(shortest_path.path_to(&4), Some(vec![4]));
        assert_eq!(shortest_path.path_to(&5), Some(vec![5, 4]));
        assert_eq!(shortest_path.path_to(&6), None);
        assert_eq!(shortest_path.path_to(&7), None);
        assert_eq!(shortest_path.path_to(&8), None);

        let mut shortest_path = ShortestPath::init(5, graph.nb_vertices());
        shortest_path.dijkstra(&graph);
        assert_eq!(shortest_path.path_to(&0), None);
        assert_eq!(shortest_path.path_to(&1), None);
        assert_eq!(shortest_path.path_to(&2), None);
        assert_eq!(shortest_path.path_to(&3), Some(vec![3, 5]));
        assert_eq!(shortest_path.path_to(&4), None);
        assert_eq!(shortest_path.path_to(&5), Some(vec![5]));
        assert_eq!(shortest_path.path_to(&6), None);
        assert_eq!(shortest_path.path_to(&7), None);
        assert_eq!(shortest_path.path_to(&8), None);

        let mut shortest_path = ShortestPath::init(6, graph.nb_vertices());
        shortest_path.dijkstra(&graph);
        assert_eq!(shortest_path.path_to(&0), None);
        assert_eq!(shortest_path.path_to(&1), None);
        assert_eq!(shortest_path.path_to(&2), None);
        assert_eq!(shortest_path.path_to(&3), Some(vec![3, 4, 6]));
        assert_eq!(shortest_path.path_to(&4), Some(vec![4, 6]));
        assert_eq!(shortest_path.path_to(&5), Some(vec![5, 4, 6]));
        assert_eq!(shortest_path.path_to(&6), Some(vec![6]));
        assert_eq!(shortest_path.path_to(&7), Some(vec![7, 6]));
        assert_eq!(shortest_path.path_to(&8), None);

        let mut shortest_path = ShortestPath::init(7, graph.nb_vertices());
        shortest_path.dijkstra(&graph);
        assert_eq!(shortest_path.path_to(&0), None);
        assert_eq!(shortest_path.path_to(&1), None);
        assert_eq!(shortest_path.path_to(&2), None);
        assert_eq!(shortest_path.path_to(&3), None);
        assert_eq!(shortest_path.path_to(&4), None);
        assert_eq!(shortest_path.path_to(&5), None);
        assert_eq!(shortest_path.path_to(&6), None);
        assert_eq!(shortest_path.path_to(&7), Some(vec![7]));
        assert_eq!(shortest_path.path_to(&8), None);
    }

    #[test]
    fn test_shortest_path_ewdag() {
        let mut graph = VecWeightedDiGraph::<usize, u16>::init(8);
        graph.add_edge(0, 1, 5);
        graph.add_edge(0, 4, 9);
        graph.add_edge(0, 7, 8);
        graph.add_edge(1, 2, 12);
        graph.add_edge(1, 3, 15);
        graph.add_edge(1, 7, 4);
        graph.add_edge(2, 3, 3);
        graph.add_edge(2, 6, 11);
        graph.add_edge(3, 6, 9);
        graph.add_edge(4, 5, 4);
        graph.add_edge(4, 6, 20);
        graph.add_edge(4, 7, 5);
        graph.add_edge(5, 2, 1);
        graph.add_edge(5, 6, 13);
        graph.add_edge(7, 2, 7);
        graph.add_edge(7, 5, 6);

        let mut shortest_path = ShortestPath::init(0, graph.nb_vertices());
        shortest_path.ewdag(&graph);
        assert_eq!(shortest_path.path_to(&0), Some(vec![0]));
        assert_eq!(shortest_path.path_to(&1), Some(vec![1, 0]));
        assert_eq!(shortest_path.path_to(&2), Some(vec![2, 5, 4, 0]));
        assert_eq!(shortest_path.path_to(&3), Some(vec![3, 2, 5, 4, 0]));
        assert_eq!(shortest_path.path_to(&4), Some(vec![4, 0]));
        assert_eq!(shortest_path.path_to(&5), Some(vec![5, 4, 0]));
        assert_eq!(shortest_path.path_to(&6), Some(vec![6, 2, 5, 4, 0]));
        assert_eq!(shortest_path.path_to(&7), Some(vec![7, 0]));

        let mut shortest_path = ShortestPath::init(7, graph.nb_vertices());
        shortest_path.ewdag(&graph);
        assert_eq!(shortest_path.path_to(&0), None);
        assert_eq!(shortest_path.path_to(&1), None);
        assert_eq!(shortest_path.path_to(&2), Some(vec![2, 7]));
        assert_eq!(shortest_path.path_to(&3), Some(vec![3, 2, 7]));
        assert_eq!(shortest_path.path_to(&4), None);
        assert_eq!(shortest_path.path_to(&5), Some(vec![5, 7]));
        assert_eq!(shortest_path.path_to(&6), Some(vec![6, 2, 7]));
        assert_eq!(shortest_path.path_to(&7), Some(vec![7]));
    }
}
