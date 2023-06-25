mod algorithm;
#[cfg(test)]
mod unit_test;
use crate::utils::read_lines;
pub use algorithm::UnionFindAlgorithm;
use std::path::Path;

/// Implementation of union-find algorithms
/// # Examples
/// ```
/// use algods::search::{UnionFind, UnionFindAlgorithm};
/// let mut uf = UnionFind::with_capacity(4, UnionFindAlgorithm::QuickUnion);
/// uf.union(0, 1);
/// uf.union(2, 3);
/// assert!(uf.connected(0, 1));
/// assert!(!uf.connected(1, 3));
/// assert_eq!(UnionFindAlgorithm::default(), UnionFindAlgorithm::QuickFind);
/// ```
/// The default algorithm is **QuickFind**
#[derive(Debug, Default)]
pub struct UnionFind {
    // number of objects in the graph/tree
    // objects are 0..(nb_objects - 1)
    nb_objects: usize,

    // union-find algorithm
    algo: UnionFindAlgorithm,

    // objects connection information data
    // each objects j is connected to ids[j]
    ids: Vec<usize>,

    // optional size of the trees the objects belong to,
    // except for QuickFind algorithm (no tree representation)
    size: Vec<usize>,
    // Some axioms of the connection relation:
    //  - reflexive: object p is connected to p
    //  - transitive: p connected to q and q connected to r => q connected to r
    //  - symmetric: p connected to q => q connected to p
}

impl UnionFind {
    /// Creates a union find instance with one object, with a `QuickFind` algorithm.
    /// # Example
    /// ```
    /// use algods::search::{UnionFind, UnionFindAlgorithm};
    /// let uf = UnionFind::new();
    /// assert_eq!(uf.len(), 1);
    /// assert_eq!(uf.algo(), UnionFindAlgorithm::QuickFind);
    /// ```
    pub fn new() -> Self {
        let nb = 1;
        Self {
            nb_objects: nb,
            algo: UnionFindAlgorithm::default(),
            ids: (0..nb).collect::<Vec<usize>>(),
            size: vec![1; nb],
        }
    }

    pub fn with_capacity(nb: usize, algorithm: UnionFindAlgorithm) -> Self {
        // complexity: O(N)
        match algorithm {
            UnionFindAlgorithm::QuickFind => Self {
                nb_objects: nb,
                algo: algorithm,
                ids: (0..nb).collect::<Vec<usize>>(),
                size: Vec::new(),
            },
            _ => Self {
                nb_objects: nb,
                algo: algorithm,
                ids: (0..nb).collect::<Vec<usize>>(),
                size: vec![1; nb],
            },
        }
    }

    /// Returns the algorithm of a union find instance.
    /// # Example
    /// ```
    /// use algods::search::{UnionFind, UnionFindAlgorithm};
    /// let uf = UnionFind::with_capacity(2, UnionFindAlgorithm::QuickUnion);
    /// assert_eq!(uf.algo(), UnionFindAlgorithm::QuickUnion);
    /// ```
    pub fn algo(&self) -> UnionFindAlgorithm {
        self.algo
    }

    /// Gives the number of objects collected.
    /// # Example
    /// ```
    /// use algods::search::{UnionFind, UnionFindAlgorithm};
    /// let uf = UnionFind::with_capacity(4, UnionFindAlgorithm::QuickUnion);
    /// assert_eq!(uf.len(), 4);
    /// ```
    pub fn len(&self) -> usize {
        self.nb_objects
    }

    /// Indicates whether or not a union find instance is empty
    /// # Example
    /// ```
    /// use algods::search::{UnionFind, UnionFindAlgorithm};
    /// let uf = UnionFind::with_capacity(0, UnionFindAlgorithm::QuickUnion);
    /// assert_eq!(uf.len(), 0);
    /// ```
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Creates a union find instance from a file with objects ids
    /// separated by a separator like `,` or `;` or `|`, etc.
    /// # Panics
    /// It panics if the ids are not positive integers.
    /// # Example
    /// ```
    /// use algods::search::{UnionFind, UnionFindAlgorithm};
    /// let uf = UnionFind::from_file("file_path", ';', UnionFindAlgorithm::QuickFind);
    /// ```
    pub fn from_file<P>(filename: P, sep: char, algo: UnionFindAlgorithm) -> Self
    where
        P: AsRef<Path>,
    {
        // Builds an Union Find object from a file
        // first element is the number of objects
        // the subsequent rows give the pairs of
        // connected objects
        let mut nb_iter = 0;
        let mut uf = UnionFind::new();

        if let Ok(lines) = read_lines(filename) {
            for (pos, line) in lines.enumerate() {
                if let Ok(row) = line {
                    let values = row.split(sep).collect::<Vec<&str>>();
                    if pos == 0 {
                        let nb_objects = values[0].parse::<usize>().unwrap();
                        uf = UnionFind::with_capacity(nb_objects, algo);
                    } else {
                        let (p, q) = (
                            values[0].parse::<usize>().unwrap(),
                            values[1].parse::<usize>().unwrap(),
                        );
                        if !uf.connected(p, q) {
                            uf.union(p, q);
                            println!("{nb_iter}");
                            nb_iter += 1
                        }
                    }
                }
            }
        }
        uf
    }

    /// Gets the id of the root of an object in a union find instance.
    /// Depending on the algorithm the root is computed differently and the complexity differs also.
    /// For `QuickFInd`, the value returned is the parent of the object, in a tree representation.
    /// For the others, the value returned is the parent or the great parent or a great great .. parent.
    /// # Example
    /// ```
    /// use algods::search::{UnionFind, UnionFindAlgorithm};
    /// let mut uf = UnionFind::with_capacity(2, UnionFindAlgorithm::WeightedQuickUnion);
    /// uf.union(0, 1);
    /// assert_eq!(uf.root(0), uf.root(1));
    /// ```
    /// # Time complexity
    /// For `QuickFind` it is expected to run in O(1).
    /// For `QuickUnion`, it is expected to run in O(N).
    /// For `WeightedQuickUnion`, it is expected to run in O(log(N)).
    /// For `WeightedQuickUnionPathComp`, it is expected to run in O(log*(N)) ~ O(1) (almost in constant time).
    pub fn root(&mut self, mut i: usize) -> usize {
        // Finding the root of an object i
        if let UnionFindAlgorithm::QuickFind = self.algo {
            self.ids[i]
        } else if let UnionFindAlgorithm::WeightedQuickUnionPathComp = self.algo {
            while i != self.ids[i] {
                // one-pass path compression
                self.ids[i] = self.ids[self.ids[i]];
                i = self.ids[i];
            }
            i
        } else {
            while i != self.ids[i] {
                i = self.ids[i];
            }
            i
        }
    }

    /// Indicates whether or not two objects in a union find instance are connected.
    /// # Example
    /// ```
    /// use algods::search::{UnionFind, UnionFindAlgorithm};
    /// let mut uf = UnionFind::with_capacity(2, UnionFindAlgorithm::WeightedQuickUnion);
    /// uf.union(0, 1);
    /// assert!(uf.connected(0, 1));
    /// ```
    /// # Time complexity
    /// For `QuickFind`, it is expected to run in O(1).
    /// For `QuickUnion`, it is expected to run in O(N).
    /// For `WeightedQuickUnion`, it is expected to run in O(log(N)).
    /// For `WeightedQuickUnionPathComp`, it is expected to run in O(log*(N)) ~ O(1) (almost in constant time).
    pub fn connected(&mut self, p: usize, q: usize) -> bool {
        if let UnionFindAlgorithm::QuickFind = self.algo {
            // complexity: O(1)
            self.ids[p] == self.ids[q]
        } else {
            // complexity: O(N) for QuickUnion,
            // O(log(N)) for the algorithms WeightedQuickUnion*
            self.root(p) == self.root(q)
        }
    }

    /// Connects two objects of a union find algorithm.
    /// # Example
    /// ```
    /// use algods::search::{UnionFind, UnionFindAlgorithm};
    /// let mut uf = UnionFind::with_capacity(4, UnionFindAlgorithm::QuickFind);
    /// uf.union(0, 1);
    /// uf.union(2, 3);
    /// assert!(uf.connected(0, 1));
    /// assert!(!uf.connected(3, 0));
    /// ```
    /// # Time complexity
    /// For `QuickFind`, it is expected to run in O(N).
    /// For `QuickUnion`, it is expected to run in O(N).
    /// For `WeightedQuickUnion`, it is expected to run in O(log(N)).
    /// For `WeightedQuickUnionPathComp`, it is expected to run in O(log*(N)) ~ O(1) (almost in constant time).
    pub fn union(&mut self, p: usize, q: usize) {
        match self.algo {
            UnionFindAlgorithm::QuickFind => {
                // complexity: O(N)
                let pid: usize = self.ids[p];
                let qid: usize = self.ids[q];
                // Connect to q all objects connected to p
                for i in 0..self.ids.len() {
                    if self.ids[i] == pid {
                        self.ids[i] = qid;
                    }
                }
            }
            UnionFindAlgorithm::QuickUnion => {
                // complexity: O(N) (tall trees)
                let i = self.root(p);
                let j = self.root(q);
                self.ids[i] = j;
            }
            _ => {
                // complexity: given roots (of complexity O(log(N)))
                // O(log(N)) or O(log*(N)) ~ O(1) with PathComp (path compression)
                let i = self.root(p);
                let j = self.root(q);
                // Put the smallest tree under the tallest one
                if self.size[i] < self.size[j] {
                    self.ids[i] = j;
                    self.size[j] += self.size[i];
                } else {
                    self.ids[j] = i;
                    self.size[i] += self.size[j];
                }
            }
        }
    }
}
