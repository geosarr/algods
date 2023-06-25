mod algorithm;
#[cfg(test)]
mod unit_test;
pub use algorithm::MergeSortAlgorithm;
use std::cmp::min;
use std::mem::replace;

/// Implementation of merge sort algorithm.
#[derive(Debug)]
pub struct MergeSort<T> {
    vec: Vec<T>,
    algo: MergeSortAlgorithm,
    // MergeSort is stable in the sense that
    // items with equal keys are not exchanged ?
}

impl<T> MergeSort<T> {
    /// Creates a new merge sort instance from a `Vec` and a merge sort algorithm
    /// (recursive or iterative (i.e bottom-up))
    /// ```
    /// use algods::sort::{MergeSort, MergeSortAlgorithm};
    /// let ms = MergeSort::init(vec![100, 20, 9, 17], MergeSortAlgorithm::BottomUp);
    /// ```
    pub fn init(v: Vec<T>, algorithm: MergeSortAlgorithm) -> Self {
        Self {
            vec: v,
            algo: algorithm,
        }
    }
}
impl<T: Ord + Clone> MergeSort<T> {
    fn is_sorted(&mut self, low: usize, high: usize) -> bool {
        if self.vec[low..high].is_empty() {
            return true;
        }
        for k in low..high {
            if self.vec[k] > self.vec[k + 1] {
                return false;
            }
        }
        true
    }

    fn merge(&mut self, mut aux_vec: Vec<T>, low: usize, high: usize, mid: usize) {
        // The halves of v, that is v[lo..mid+1] and v[mid+1..], should be sorted
        assert!(self.is_sorted(low, mid));
        assert!(self.is_sorted(mid + 1, high));

        aux_vec = self.vec.clone();
        let mut i = low;
        let mut j = mid + 1;
        for k in low..high + 1 {
            if i > mid {
                let _ = replace(&mut self.vec[k], aux_vec[j].clone());
                j += 1;
            } else if j > high {
                let _ = replace(&mut self.vec[k], aux_vec[i].clone());
                i += 1;
            } else if aux_vec[j] <= aux_vec[i] {
                let _ = replace(&mut self.vec[k], aux_vec[j].clone());
                j += 1;
            } else {
                let _ = replace(&mut self.vec[k], aux_vec[i].clone());
                i += 1;
            }
        }
        assert!(self.is_sorted(low, high));
    }

    fn recursive_sort(&mut self, aux_vec: Vec<T>, low: usize, high: usize) {
        let mid = low + (high - low) / 2;
        if high <= low {
            return;
        }
        self.recursive_sort(aux_vec.clone(), low, mid);
        self.recursive_sort(aux_vec.clone(), mid + 1, high);
        self.merge(aux_vec, low, high, mid);
    }

    /// Sorts a `Vec` using merge sort algorithm. It moves the MergeSort.
    /// ```
    /// use algods::sort::{MergeSort, MergeSortAlgorithm};
    /// let mut v = vec![10, 20, 9, 17];
    /// let ms = MergeSort::init(v.clone(), MergeSortAlgorithm::Recursive);
    /// v.sort_unstable();
    /// assert_eq!(ms.into_sorted_vec(), v);
    /// ```
    pub fn into_sorted_vec(mut self) -> Vec<T> {
        let aux_vec = Vec::<T>::new();
        let n = self.vec.len();
        match self.algo {
            MergeSortAlgorithm::Recursive => {
                self.recursive_sort(aux_vec, 0, n - 1);
            }
            MergeSortAlgorithm::BottomUp => {
                let mut sizes: Vec<usize> = (1..n).collect::<Vec<usize>>();
                sizes.retain(|x| x.is_power_of_two());
                // println!("{:?}", sizes);
                // Looping over subvectors of sizes equal to
                // powers of 2 that are >= 1 and < N
                for size in &sizes {
                    let double_size = size + size;
                    // println!("{:?}", self.vec);
                    for low in (0..n - size).step_by(double_size) {
                        self.merge(
                            aux_vec.clone(),
                            low,
                            min(low + double_size - 1, n - 1),
                            low + size - 1,
                        )
                    }
                }
            }
        }
        self.vec
    }
}
