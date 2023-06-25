#[cfg(test)]
mod unit_test;
use std::mem::replace;

/// Implementation of insertion sort algorithm.
#[derive(Debug)]
pub struct InsertionSort<T> {
    vec: Vec<T>,
}

impl<T: Clone + Ord> InsertionSort<T> {
    /// Creates a new insertion sort instance from a `Vec`
    /// ```
    /// use algods::sort::InsertionSort;
    /// let i_s = InsertionSort::init(vec![100, 20, 9, 17]);
    /// ```
    pub fn init(v: Vec<T>) -> Self {
        Self { vec: v }
    }

    /// Sorts a `Vec` using insertion sort algorithm. It moves the InsertionSort.
    /// ```
    /// use algods::sort::InsertionSort;
    /// let mut v = vec![100, 20, 9, 17];
    /// let i_s = InsertionSort::init(v.clone());
    /// v.sort_unstable();
    /// assert_eq!(i_s.into_sorted_vec(), v);
    /// ```   
    pub fn into_sorted_vec(mut self) -> Vec<T> {
        let n = self.vec.len();
        for i in 0..n {
            let mut j = i;
            while j > 0 && self.vec[j] < self.vec[j - 1] {
                let val = self.vec[j - 1].clone();
                self.vec[j - 1] = replace(&mut self.vec[j], val);
                j -= 1;
            }
        }
        self.vec
    }
}
