#[cfg(test)]
mod unit_test;
use algods::search::{UnionFind, UnionFindAlgorithm};
use rand::prelude::*;
use std::collections::HashMap;
use std::thread;
use std::thread::JoinHandle;

#[derive(Debug)]
struct Percolation {
    // number of rows or columns of a squared grid
    grid_size: usize,
    // Union-Find object
    uf: UnionFind,
    // indicates whether or not a site p is open or not
    state: Vec<bool>,
    // Remark:
    // each site (i,j) corresponds to an object p = i*self.grid_size + j
    // for 0 <= i,j <= n-1 such that 0 <= p <= n^2 - 1
}
impl Default for Percolation {
    fn default() -> Self {
        Self::new()
    }
}

impl Percolation {
    pub fn new() -> Self {
        Self {
            grid_size: 50,
            uf: UnionFind::new(),
            state: Vec::new(),
        }
    }

    pub fn with_capacity(n: usize, algo: UnionFindAlgorithm) -> Self {
        // complexity O(n^2)
        Self {
            grid_size: n,
            uf: UnionFind::with_capacity(n * n, algo),
            state: vec![false; n * n],
        }
    }

    fn open(&mut self, row: usize, col: usize) {
        if !self.is_open(row, col) {
            // open the site
            let site_id = row * self.grid_size + col;
            self.state[site_id] = true;

            // connect the site (row, col) to its open neighbors sites
            let nb = self.grid_size;
            if row > 0 && self.is_open(row - 1, col) {
                let neighbor_site_id = (row - 1) * nb + col;
                if !self.uf.connected(site_id, neighbor_site_id) {
                    self.uf.union(site_id, neighbor_site_id);
                }
            }
            if col > 0 && self.is_open(row, col - 1) {
                let neighbor_site_id = row * nb + col - 1;
                if !self.uf.connected(site_id, neighbor_site_id) {
                    self.uf.union(site_id, neighbor_site_id);
                }
            }
            if row < nb - 1 && self.is_open(row + 1, col) {
                let neighbor_site_id = (row + 1) * nb + col;
                if !self.uf.connected(site_id, neighbor_site_id) {
                    self.uf.union(site_id, neighbor_site_id);
                }
            }
            if col < nb - 1 && self.is_open(row, col + 1) {
                let neighbor_site_id = row * nb + col + 1;
                if !self.uf.connected(site_id, neighbor_site_id) {
                    self.uf.union(site_id, neighbor_site_id);
                }
            }
        }
    }

    fn is_open(&self, row: usize, col: usize) -> bool {
        self.state[row * self.grid_size + col]
    }

    fn is_full(&mut self, row: usize, col: usize) -> bool {
        let nb = self.grid_size;
        if self.is_open(row, col) {
            let site_id = row * nb + col;
            // check if there is any path of open sites connected the current site (row, col)
            // and starting from at least one open top site
            // NB: top sites are the self.uf.nb_objects first objects in self.uf.ids
            // let root = self.uf.root(site_id);
            // let top = (0..nb).collect::<Vec<usize>>();
            // top.iter().any(|&i| self.uf.root(i) == root)
            let top = (0..nb).collect::<Vec<usize>>();
            top.iter().any(|&i| self.uf.connected(i, site_id))
        } else {
            false
        }
    }

    fn number_of_open_sites(&self) -> usize {
        // counts the number of open sites
        self.state.iter().map(|&open| usize::from(open)).sum()
    }

    fn is_percolated(&mut self) -> bool {
        // checks whether or not at least one of the bottom site is full
        let nb = self.grid_size;
        let bottom = ((nb - 1) * nb..nb * nb).collect::<Vec<usize>>();
        bottom.iter().any(|&i| self.is_full(i / nb, i % nb))
    }

    pub fn threshold(&mut self) -> f32 {
        let mut rng = rand::thread_rng();
        let nb = self.grid_size;
        let candidates = (0..nb * nb).collect::<Vec<usize>>();
        while !self.is_percolated() {
            // choose a random site
            let site_id = candidates.choose(&mut rng).unwrap();
            let (row, col) = (site_id / nb, site_id % nb);
            // open the chosen site if not opened
            self.open(row, col);
        }
        (self.number_of_open_sites() as f32) / ((nb * nb) as f32)
    }
}

#[derive(Debug)]
pub struct PercolationStats {
    // number of rows or columns of a squared grid
    grid_size: usize,
    // Union-Find algorithm
    algo: UnionFindAlgorithm,
    // number of times to run the Percolation algorithm
    n_trials: usize,
    // the thresholds of the Percolation system simulated n_trials times
    results: Option<Vec<f32>>,
}
impl Default for PercolationStats {
    fn default() -> Self {
        Self::new()
    }
}
impl PercolationStats {
    pub fn new() -> Self {
        Self {
            grid_size: 50,
            algo: UnionFindAlgorithm::default(),
            n_trials: 10,
            results: None,
        }
    }

    pub fn with_capacity(n: usize, algorithm: UnionFindAlgorithm, trials: usize) -> Self {
        Self {
            grid_size: n,
            algo: algorithm,
            n_trials: trials,
            results: None,
        }
    }
    pub fn compute(&mut self) {
        let grid_size: usize = self.grid_size;
        let algo: UnionFindAlgorithm = self.algo;
        let n_trials: usize = self.n_trials;
        let mut handles: Vec<JoinHandle<f32>> = Vec::new();

        for i in 0..n_trials {
            let id = i + 1;
            println!("Running computation {id} over {n_trials}");
            let mut percolation = Percolation::with_capacity(grid_size, algo);
            let handle = thread::spawn(move || percolation.threshold());
            handles.push(handle)
        }

        self.results = Some(
            handles
                .into_iter()
                .map(|h| h.join().unwrap())
                .collect::<Vec<f32>>(),
        );
    }

    pub fn mean(&self) -> f32 {
        if let Some(results) = &self.results {
            results.iter().sum::<f32>() / (self.n_trials as f32)
        } else {
            println!("No observation for mean PercolationStats");
            f32::NAN
        }
    }

    pub fn stddev(&self) -> f32 {
        if let Some(results) = &self.results {
            let trials = self.n_trials as f32;
            let avg = self.mean();
            let sum_sq_avg_dist = results.iter().map(|&i| (i - avg).powf(2.0)).sum::<f32>();
            if self.n_trials > 1 {
                (sum_sq_avg_dist / (trials - 1.0)).sqrt()
            } else {
                println!("Not enough observations (<= 1) for stddev PercolationStats");
                f32::NAN
            }
        } else {
            println!("No observation for stddev PercolationStats");
            f32::NAN
        }
    }

    pub fn conf_interval(&self) -> HashMap<&str, f32> {
        let avg = self.mean();
        let std = self.stddev();
        let trials = self.n_trials as f32;
        let low = avg - ((1.96 * std) / trials);
        let up = avg + ((1.96 * std) / trials);
        HashMap::from([("low", low), ("up", up)])
    }
}
