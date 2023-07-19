#[cfg(test)]
mod unit_test;
use bitvec::field::BitField;
pub use bitvec::prelude::{bits, BitSlice, Lsb0, Msb0};
pub use bitvec::vec::BitVec;
use std::fs::File;
use std::io;

/// Compresses a sequence of bits using a `BitVec` representation of bits
/// by means of the run-length algorithm
pub struct RunLength<T, O>
where
    T: bitvec::store::BitStore,
    O: bitvec::order::BitOrder,
{
    bits: BitVec<T, O>,
}
impl<T: bitvec::store::BitStore, O: bitvec::order::BitOrder> Default for RunLength<T, O> {
    fn default() -> Self {
        Self::new()
    }
}
impl<T, O> RunLength<T, O>
where
    T: bitvec::store::BitStore,
    O: bitvec::order::BitOrder,
{
    /// Creates an empty structure to hold bits
    /// # Examples
    /// ```
    /// use algods::compression::*;
    /// let rl = RunLength::<usize, Msb0>::new();
    /// assert_eq!(rl.len(), 0);
    /// ```
    pub fn new() -> Self {
        Self {
            bits: BitVec::new(),
        }
    }
    /// Returns the number of bits in the data structure
    /// # Examples
    /// ```
    /// use algods::compression::*;
    /// let bits = BitVec::from_bitslice(bits![1, 0, 0, 1]);
    /// let rl = RunLength::init(bits);
    /// assert_eq!(rl.len(), 4);
    /// ```
    pub fn len(&self) -> usize {
        self.bits.len()
    }
    /// Tells whether or not the structure is empty
    /// ```
    /// use algods::compression::*;
    /// let rl = RunLength::<u8, Msb0>::new();
    /// assert!(rl.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    /// Creates a bits holding structure from a `BitVec` structure
    /// # Examples
    /// ```
    /// use algods::compression::*;
    /// let bits = BitVec::from_bitslice(bits![1, 1, 1, 0, 1, 0]);
    /// let rl = RunLength::init(bits);
    /// assert_eq!(rl.len(), 6);
    /// ```
    pub fn init(bit_vec: BitVec<T, O>) -> Self {
        Self { bits: bit_vec }
    }
    /// Returns a reference to the bits in the structure
    /// # Examples
    /// ```
    /// use algods::compression::*;
    /// let bits = BitVec::from_bitslice(bits![1, 1, 1, 0, 1, 0]);
    /// let replicated_bits = BitVec::from_bitslice(bits![1, 1, 1, 0, 1, 0]);
    /// let rl = RunLength::init(bits);
    /// assert_eq!(rl.bits(), &replicated_bits);
    /// ```
    pub fn bits(&self) -> &BitVec<T, O> {
        &self.bits
    }
    /// Adds a single bit to the structure.
    /// # Examples
    /// ```
    /// use algods::compression::*;
    /// let bits = BitVec::from_bitslice(bits![0, 0, 1]);
    /// let mut rl = RunLength::init(bits);
    /// rl.push(false);
    /// let expected_bits_ref = &BitVec::from_bitslice(bits![0, 0, 1, 0]);
    /// assert_eq!(rl.bits(), expected_bits_ref);
    /// ```
    pub fn push(&mut self, bit: bool) {
        self.bits.push(bit)
    }
    /// Adds a slice of elements to the structure.
    /// # Examples
    /// ```
    /// use algods::compression::*;
    /// let mut rl = RunLength::<u8, Msb0>::new();
    /// rl.extend_from_raw_slice(&[1, 2]);
    /// let bits = bits![0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0];
    /// let expected_bits_ref = &BitVec::from_bitslice(bits);
    /// assert_eq!(rl.bits(), expected_bits_ref);
    /// ```
    pub fn extend_from_raw_slice(&mut self, slice: &[T]) {
        self.bits.extend_from_raw_slice(slice)
    }
    /// Compresses the bits in the structure, returning the sequence of counts
    /// of consecutive 0's and consecutive 1's, along with the number of bits in the initial sequence of bits.
    /// # Examples
    /// ```
    /// use algods::compression::*;
    /// let bits = BitVec::from_bitslice(bits![0, 1, 1, 1]);
    /// let run_length = RunLength::init(bits);
    /// let (compressed_bits, init_len) = run_length.compress();
    /// assert_eq!(init_len, 4);
    /// assert_eq!(compressed_bits, vec![1,3]);
    /// ```
    pub fn compress(&self) -> (Vec<usize>, usize) {
        let len = self.bits.len();
        let mut comp = Vec::<usize>::with_capacity(len);
        if len > 0 {
            let mut old = self.bits[0];
            let mut run = 0;
            if self.bits[0] {
                // The first bit is 1
                comp.push(0);
            }
            for k in 0..len {
                let bit = self.bits[k];
                if bit == old {
                    run += 1;
                } else {
                    comp.push(run);
                    run = 1;
                    old = !old;
                }
            }
            comp.push(run);
        }
        (comp, len)
    }
    /// Expands the bits in the structure, retrieving the initial sequence of bits
    /// # Examples
    /// ```
    /// use algods::compression::*;
    /// let bits = BitVec::from_bitslice(bits![0, 0, 1, 1, 1]);
    /// let run_length = RunLength::init(bits);
    /// let (compressed_bits, init_len) = run_length.compress();
    /// let expanded_bits = run_length.expand(compressed_bits, 8);
    /// assert_eq!(&expanded_bits, run_length.bits());
    /// ```
    pub fn expand(&self, comp: Vec<usize>, capacity: usize) -> BitVec<T, O> {
        let len = comp.len();
        assert!(capacity >= len);
        let mut exp = BitVec::<T, O>::with_capacity(capacity);
        for (pos, run) in comp.iter().enumerate() {
            let bit = pos % 2 == 1;
            for _ in 0..*run {
                exp.push(bit);
            }
        }
        exp
    }
}
impl<T, O> RunLength<T, O>
where
    T: bitvec::store::BitStore,
    O: bitvec::order::BitOrder,
    BitSlice<T, O>: BitField,
{
    /// Copies a file bitwise.
    /// # Examples
    /// ```
    /// use algods::compression::*;
    /// let file_path: &str = "path/to/file" ;
    /// // let run_length = RunLength::<u8, Lsb0>::from_file(file_path);
    /// // let (comp, init_nb_bits) = run_length.compress();
    /// // let original_bits = run_length.expand(comp, init_nb_bits);
    /// // assert!(run_length.bits() == &original_bits);
    /// ```
    pub fn from_file(path: &str) -> Self {
        let mut writer = BitVec::<T, O>::new();
        let reader = File::open(path);
        match reader {
            Ok(mut file) => match io::copy(&mut file, &mut writer) {
                Ok(_) => Self { bits: writer },
                Err(_) => panic!("Failed to copy file"),
            },
            Err(error) => panic!("{error:?}"),
        }
    }
}
