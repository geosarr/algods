#[cfg(test)]
mod unit_test;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::cmp::Ordering;
use std::mem::replace;

/// Implementation of quick sort algorithm
#[derive(Debug)]
pub struct QuickSort<T> {
    vec: Vec<T>,
    // QuickSort is unstable in the sense that
    // items with equal keys can be exchanged ?
}

impl<T: Ord + Clone> QuickSort<T> {
    /// Creates a new quick sort instance from a `Vec`.
    /// ```
    /// use algods::sort::QuickSort;
    /// let ms = QuickSort::init(vec![100, 20, 9, 17]);
    /// ```
    pub fn init(v: Vec<T>) -> Self {
        Self { vec: v }
    }

    fn partition(&mut self, low: usize, high: usize) -> usize {
        // Finds a partitioning index j, i.e such that elements at the left of index j
        // are no greater than element j and elements at the right of index j are no larger
        // than element j.
        // run time complexity O(N)
        let mut i = low + 1;
        let mut j = high;
        loop {
            // find item on left to swap
            while self.vec[i] <= self.vec[low] {
                i += 1;
                if i >= high {
                    break;
                }
            }
            // find item on right to swap
            while self.vec[low] <= self.vec[j] {
                j -= 1;
                if j <= low {
                    break;
                }
            }
            // check if positions/pointers cross
            if i >= j {
                break;
            }
            // exchange i^th and j^th object to respect partitioning
            let n = self.vec[j].clone();
            self.vec[j] = replace(&mut self.vec[i], n);
        }
        // swap with partitioning item
        let n = self.vec[j].clone();
        self.vec[j] = replace(&mut self.vec[low], n);
        j
    }

    fn sort(&mut self) {
        // shuffle garantees performance ?
        let mut rng = thread_rng();
        self.vec.shuffle(&mut rng);
        self.recursive_sort(0, self.vec.len() - 1);
    }

    fn recursive_sort(&mut self, low: usize, high: usize) {
        if high <= low {
            return;
        }
        let j: usize = self.partition(low, high);
        if j >= 1 && j < self.vec.len() - 1 {
            self.recursive_sort(low, j - 1);
            self.recursive_sort(j + 1, high);
        } else if j == 0 {
            self.recursive_sort(j + 1, high);
        } else {
            self.recursive_sort(low, j - 1);
        }
    }

    /// Sorts a `Vec` using quick sort algorithm. It moves the QuickSort.
    /// ```
    /// use algods::sort::QuickSort;
    /// let mut v = vec![100, 20, 9, 17];
    /// let qs = QuickSort::init(v.clone());
    /// v.sort_unstable();
    /// assert_eq!(qs.into_sorted_vec(), v);
    /// ```   
    pub fn into_sorted_vec(mut self) -> Vec<T> {
        self.sort();
        self.vec
    }

    /// Selects the kth largest element in the `Vec` instantiating the QuickSort using the
    /// quick select algorithm
    /// ```
    /// use algods::sort::QuickSort;
    /// let mut qs = QuickSort::init(vec![30, 13, 90, 50, 47, 100]);
    /// assert_eq!(qs.select(3), 50);
    /// ```
    pub fn select(&mut self, k: usize) -> T {
        // Selects the k^th largest element in self.vec (quick select algorithm)
        // in linear time on average for median (k = N/2)
        let mut rng = thread_rng();
        self.vec.shuffle(&mut rng);
        let mut low = 0;
        let mut high = self.vec.len() - 1;
        while high > low {
            let j = self.partition(low, high);
            match j.cmp(&k) {
                // the k^th largest is on the right of element j
                Ordering::Less => low = j + 1,
                // the k^th largest is on the left of element j
                Ordering::Greater => high = j - 1,
                // j = k, we are done
                Ordering::Equal => return self.vec[k].clone(),
            }
        }
        self.vec[k].clone()
    }
}
