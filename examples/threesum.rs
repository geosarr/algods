use algods::search::binary_search;

pub struct ThreeSum<T> {
    // Solves the problem find among distincts values a1, a2, .., an,
    // how many triplets sump up to a target value. The run time complexity is O(N^2log(N))
    target: T,
    // list the values ak
    vec: Vec<T>,
}

impl<T: Ord + std::ops::Add<Output = T> + std::ops::Sub<Output = T> + Copy> ThreeSum<T> {
    pub fn init(t: T, v: Vec<T>) -> Self {
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

fn main() {
    use algods::utils::{gen_vec_rand_int, RandKind};
    use std::thread;
    use std::thread::JoinHandle;

    let vec = gen_vec_rand_int(20, RandKind::Range);
    let n = vec.len();
    let mut threesum = ThreeSum::<isize>::init(0, vec);
    let mut handles: Vec<JoinHandle<isize>> = Vec::new();
    for i in 0..n {
        let a = threesum.vec[i];
        for j in (i + 1)..n {
            let b = threesum.vec[j];
            for k in (j + 1)..n {
                let c = threesum.vec[k];
                let handle = thread::spawn(move || a + b + c);
                handles.push(handle);
            }
        }
    }
    let sums = handles
        .into_iter()
        .map(|h| h.join().unwrap())
        .collect::<Vec<isize>>();
    for s in &sums {
        threesum.target = *s;
        assert!(1 <= threesum.run());
    }
}
