#[cfg(test)]
mod unit_test;
use std::result::Result;

/// Gets the index of a key in a slice if key is in vec
/// Otherwise returns the index where it should be put
/// to keep the slice ordered
/// # Requirement
/// The keys in the slice should be ordered in ascending order
/// # Time complexity
/// It is expected to run in O(log(N))
pub fn binary_search<T>(key: T, vec: &[T]) -> Result<usize, usize>
where
    T: Ord,
{
    if !vec.is_empty() {
        let mut high = vec.len() - 1;
        let mut low = 0;

        while low <= high {
            let mid = (high + low) / 2;
            if vec[mid] < key {
                low = mid + 1;
            } else if vec[mid] > key && mid > 0 {
                high = mid - 1;
            }
            // mid = 0 <=> high=0=low or (high=1 and low=0)
            // when mid=0 (mid-1 not working since mid is usize)
            // hence the following conditions
            else if vec[mid] > key && mid == 0 {
                return Err(mid);
            } else {
                return Ok(mid);
            }
        }
        Err(low)
    } else {
        Err(0)
    }
}
