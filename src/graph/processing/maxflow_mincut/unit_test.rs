#[cfg(test)]

mod tests {
    use super::super::{FlowNetwork, FordFulkerson};

    #[test]
    fn test_ford_fulkerson() {
        let mut network = FlowNetwork::init(6);
        // From CLRS book
        network.add_edge(0, 1, 16);
        network.add_edge(0, 2, 13);
        network.add_edge(1, 3, 12);
        network.add_edge(2, 1, 4);
        network.add_edge(2, 4, 14);
        network.add_edge(3, 2, 9);
        network.add_edge(3, 5, 20);
        network.add_edge(4, 3, 7);
        network.add_edge(4, 5, 4);
        let mut ff = FordFulkerson::new();
        ff.find_flows(&mut network, 0, 5);
        assert_eq!(Some(23), ff.max_flow());
    }
}
