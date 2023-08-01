use algods::graph::*;
fn main() {
    // Example from thealgorists.com
    // 0:USD, 1:EUR, 2:GBP, 3:CHF, 4:CAD
    let graph = VecWeightedDiGraph::<u8, f64>::from_vec(vec![
        (0, 1, 0.741),
        (0, 2, 0.657),
        (0, 3, 1.061),
        (0, 4, 1.005),
        (1, 0, 1.349),
        (1, 2, 0.888),
        (1, 3, 1.433),
        (1, 4, 1.366),
        (2, 0, 1.521),
        (2, 1, 1.126),
        (2, 3, 1.614),
        (2, 4, 1.538),
        (3, 0, 0.942),
        (3, 1, 0.698),
        (3, 2, 0.619),
        (3, 4, 0.953),
        (4, 0, 0.995),
        (4, 1, 0.732),
        (4, 2, 0.650),
        (4, 3, 1.049),
    ]);
    let mut shortest_path = processing::ShortestPath::init(0, graph.nb_vertices());
    shortest_path.spfa(&graph);
    println!("{:?}", shortest_path.dist_to(&0));
    println!("{:?}", shortest_path.dist_to(&1));
    println!("{:?}", shortest_path.dist_to(&2));
    println!("{:?}", shortest_path.dist_to(&3));
}
