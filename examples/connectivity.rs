use algods::search::{UnionFind, UnionFindAlgorithm};

fn main() {
    // define n objects to connect, and an union-find algorithm
    let n = 5;
    let algo = UnionFindAlgorithm::WeightedQuickUnion;
    let mut uf = UnionFind::with_capacity(n, algo);

    // build the connection graph
    uf.union(0, 4);
    uf.union(2, 3);
    uf.union(1, 4);
    println!("{:?}", uf);

    // test the connectivity
    assert!(uf.connected(0, 1));
    assert!(!uf.connected(2, 4));
}
