#[cfg(test)]
mod unit_test;
use crate::data_structure::stack::Stack;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

/// Implementation of a separate chaining based symbol table
/// # Example
/// ```
/// use algods::data_structure::SepChainTable;
/// let mut st = SepChainTable::<usize, &str>::new();
/// st.insert(0, "0");
/// st.insert(1, "1");
/// assert_eq!(st.len(), 2);
/// ```
#[derive(Debug)]
pub struct SepChainTable<T, U> {
    // number of chains in the hash table
    chains: usize,
    // list of chains
    vec: Vec<Stack<(T, U)>>,
    // number of elements in the table
    len: usize,
    // Remarks:
    // - since collision is almost unavoidable (birthday "paradox"), the idea is to put keys that have the same hash
    // in the same linked list (Stack here) and to search an element, it suffices to go through the stack to find
    // the key and its value
    // - to ensure constant time for search (one should update the length of the list of chains (chains field))
    //   so that N/chains ~ constant (where N is the number of keys in the table) with resizing strategies (e.g doubling and halving)
}
impl<T, U> SepChainTable<T, U> {
    fn _halve(&mut self) {
        // reduce the size of the chain to free space
        self.vec.truncate(self.vec.len() / 2);
        self.chains /= 2;
    }
    /// Gives the number of (key, value) pairs in the symbol table.
    /// # Example
    /// ```
    /// use algods::data_structure::SepChainTable;
    /// let mut st = SepChainTable::<isize, usize>::new();
    /// st.insert(-2, 3);
    /// assert_eq!(st.len(), 1);
    /// ```
    pub fn len(&self) -> usize {
        self.len
    }

    /// Indicates whether or not the symbol table is empty
    /// # Example
    /// ```
    /// use algods::data_structure::SepChainTable;
    /// let mut st = SepChainTable::<isize, usize>::new();
    /// st.insert(-1, 10);
    /// assert!(!st.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
impl<T: Clone, U: Clone> SepChainTable<T, U> {
    fn _double(&mut self) {
        // doubles the number of chains
        self.vec.resize(2 * self.chains, Stack::new());
        self.chains *= 2;
    }
}
impl<T: Hash, U> SepChainTable<T, U> {
    fn calculate_hash(t: &T) -> u64 {
        let mut s = DefaultHasher::new();
        t.hash(&mut s);
        s.finish()
    }
    fn hash(&self, key: &T) -> usize {
        // makes the default hash within range [0, chains-1]
        let h = Self::calculate_hash(key) as usize;
        h.rem_euclid(self.chains)
        // h % self.chains
    }
}
impl<T: Hash + Clone, U: Clone> Default for SepChainTable<T, U> {
    fn default() -> Self {
        Self::new()
    }
}
impl<T: Hash + Clone, U: Clone> SepChainTable<T, U> {
    /// Creates an empty symbol table instance.
    /// # Example
    /// ```
    /// use algods::data_structure::SepChainTable;
    /// let st = SepChainTable::<usize, usize>::new();
    /// assert_eq!(st.len(), 0);
    /// ```
    pub fn new() -> Self {
        let nb_chains = 97;
        Self {
            chains: nb_chains,
            vec: vec![Stack::new(); nb_chains],
            len: 0,
        }
    }
    /// Creates a new symbol table from an initial (key, value) pair.
    /// # Example
    /// ```
    /// use algods::data_structure::SepChainTable;
    /// let st = SepChainTable::init(1, 0, "sep_chain");
    /// assert_eq!(st.len(), 1);
    /// ```
    pub fn init(_chains: usize, key: T, value: U) -> Self {
        assert!(_chains > 0);
        let mut symbol_table = Self {
            chains: _chains,
            vec: vec![Stack::new(); _chains],
            len: 0,
        };
        let index = symbol_table.hash(&key);
        symbol_table.vec[index] = Stack::init((key, value));
        symbol_table.len = 1;
        symbol_table
    }
}
impl<T: Hash + Clone + Eq, U: Clone + Eq> SepChainTable<T, U> {
    /// Inserts a (key, value) pair into the symbol table.
    /// # Example
    /// ```
    /// use algods::data_structure::SepChainTable;
    /// let mut st = SepChainTable::<isize, isize>::new();
    /// st.insert(1, -3);
    /// st.insert(-2, 10);
    /// assert_eq!(st.len(), 2);
    /// assert_eq!(st.get(&1), Some(&-3));
    /// ```
    pub fn insert(&mut self, key: T, value: U) {
        // works if chains is small enough so that all index in u64
        // can be converted to usize (see self.hash() function)
        let index = self.hash(&key);
        let stack_len = self.vec[index].len();
        let mut stack = self.vec[index].first_mut();
        let mut c = 0;
        while c < stack_len {
            let temp_stack = stack.as_mut().unwrap();
            if key == temp_stack.item().0 {
                // replace its value
                temp_stack.item_mut().1 = value.clone();
                break;
            }
            stack = temp_stack.next_mut();
            c += 1;
        }
        if c >= stack_len {
            self.vec[index].push((key, value));
            // self.double();
        };
        self.len += 1;
    }
}
impl<T: Eq + Hash, U: Eq> SepChainTable<T, U> {
    /// Tests whether or not the symbol table contains a given key.
    /// # Example
    /// ```
    /// use algods::data_structure::SepChainTable;
    /// let mut st = SepChainTable::<usize, &str>::new();
    /// st.insert(1, "1");
    /// assert!(st.contains(&1));
    /// assert!(!st.contains(&0));
    /// ```
    pub fn contains(&self, key: &T) -> bool {
        self.get(key).is_some()
    }
    /// Returns some reference to the value associated to a key, if any.
    /// Otherwise returns `None`.
    /// # Example
    /// ```
    /// use algods::data_structure::SepChainTable;
    /// let mut st = SepChainTable::<usize, &str>::new();
    /// st.insert(1, "1");
    /// assert_eq!(st.get(&1), Some(&"1"));
    /// st.insert(1, "0");
    /// assert_eq!(st.get(&1), Some(&"0"));
    /// assert_eq!(st.get(&2), None);
    /// ```
    pub fn get(&self, key: &T) -> Option<&U> {
        // run time complexity on average O(N/chains)
        // because on average the stacks are equally likely to
        // have the same number of keys due to the coupon collector
        // and load balancing properties
        let index = self.hash(key);
        let mut stack = self.vec[index].first();
        while stack.is_some() {
            let temp_stack = stack.as_ref().unwrap();
            if key == &temp_stack.item().0 {
                return Some(&temp_stack.item().1);
            }
            stack = temp_stack.next();
        }
        None
    }
}
