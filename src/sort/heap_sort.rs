#[cfg(test)]
mod unit_test;
use std::collections::BinaryHeap;
use std::mem::replace;

/// Implementation of binary heap sort with the standard library
#[derive(Debug)]
pub struct BinaryHeapSort<T> {
    heap: BinaryHeap<T>,
}

impl<T: Ord> BinaryHeapSort<T> {
    /// Creates a new binary heap instance from a `Vec`
    /// ```
    /// use algods::sort::BinaryHeapSort;
    /// let bhs = BinaryHeapSort::init(vec![10, 20, 9, 17]);
    /// ```
    pub fn init(v: Vec<T>) -> Self {
        Self {
            heap: BinaryHeap::from(v),
        }
    }

    /// Sorts a `Vec` using heap sort algorithm. It moves the BinaryHeapSort.
    /// ```
    /// use algods::sort::BinaryHeapSort;
    /// let mut v = vec![10, 20, 9, 17];
    /// let bhs = BinaryHeapSort::init(v.clone());
    /// v.sort_unstable();
    /// assert_eq!(bhs.into_sorted_vec(), v);
    /// ```
    pub fn into_sorted_vec(self) -> Vec<T> {
        self.heap.into_sorted_vec()
    }
}

/// Implementation of heap sort relatively from scratch using a max oriented binary heap
#[derive(Debug)]
pub struct HeapSort<T> {
    vec: Vec<T>,
}

impl<T: Default + Clone> HeapSort<T> {
    /// Creates a new binary heap instance from a `Vec`
    /// ```
    /// use algods::sort::HeapSort;
    /// let bhs = HeapSort::init(vec![10, 20, 9, 17]);
    /// ```
    pub fn init(mut v: Vec<T>) -> Self {
        let mut new_vec = vec![T::default()]; // index 0 is not used
        new_vec.append(&mut v);
        Self { vec: new_vec }
    }
}

impl<T: Ord + Clone> HeapSort<T> {
    fn sink(&mut self, mut k: usize, n: usize) {
        // moves data at position k down in the "tree" following the
        // Power struggle principle: Better nodes are promoted
        // Nodes beyond node n are untouched.
        // run time complexity O(log(N))
        while 2 * k < n {
            let mut j = 2 * k;
            // find the largest child of node k
            if j < n - 1 && self.vec[j] < self.vec[j + 1] {
                j += 1;
            }
            // compare it to node k
            if self.vec[k] >= self.vec[j] {
                break;
            }
            // exchange them if it is larger than node k
            let val = self.vec[k].clone();
            self.vec[k] = replace(&mut self.vec[j], val);
            k = j;
        }
    }

    fn sink_all(&mut self) {
        // design a Binary Heap from self.vec
        // run time complexity O(N log(N))
        let n = self.vec.len();
        for j in (1..(n + 1) / 2).rev() {
            let new_j = j;
            self.sink(new_j, n);
        }
    }

    fn sort_down(mut self) -> Vec<T> {
        let mut n = self.vec.len();
        // run time complexity O(N log(N))
        while n > 1 {
            let val = self.vec[1].clone();
            self.vec[1] = replace(&mut self.vec[n - 1], val);
            n -= 1;
            self.sink(1, n);
        }
        self.vec[1..].into()
    }

    /// Sorts a `Vec` using heap sort algorithm. It moves the HeapSort.
    /// ```
    /// use algods::sort::HeapSort;
    /// let mut v = vec![10, 20, 9, 17];
    /// let bhs = HeapSort::init(v.clone());
    /// v.sort_unstable();
    /// assert_eq!(bhs.into_sorted_vec(), v);
    /// ```   
    pub fn into_sorted_vec(mut self) -> Vec<T> {
        // run time complexity O(N log(N))
        self.sink_all(); // creates a binary heap
        self.sort_down() // sorts the objects in the binary heap
    }
}
