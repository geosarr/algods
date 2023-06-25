#[cfg(test)]
mod unit_test;
use crate::search::binary_search;

pub struct ThreeSum {
    // Solves the problem find among distincts values a1, a2, .., an,
    // how many triplets sump up to a target value. The run time complexity is O(N^2log(N))
    target: isize,
    // list the values ak
    vec: Vec<isize>,
}

impl ThreeSum {
    pub fn init(t: isize, v: Vec<isize>) -> Self {
        Self { target: t, vec: v }
    }
    pub fn run(&mut self) -> usize {
        // run time complexity O(N^2 log(N))
        let mut res = 0;
        // the binary search algo needs the values to be
        // sorted first in ascending order
        self.vec.sort_unstable();
        let n = self.vec.len();
        for i in 0..n {
            for j in (i + 1)..n {
                // binary search target-vec[i]-vec[j] in vec[(j+1)..]
                let key = self.target - self.vec[i] - self.vec[j];
                let index = binary_search(key, &self.vec[(j + 1)..]);
                if index.is_ok() {
                    // a solution is found
                    res += 1;
                }
            }
        }
        // println!("Result:");
        res
    }
}
