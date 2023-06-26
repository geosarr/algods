#[cfg(test)]
mod unit_test;
use std::cmp::Ordering;
use std::collections::BTreeMap;

/// Implementation a tree map with the standard library
/// # Examples
/// ```
/// use algods::data_structure::BTreeTable;
/// let mut bt = BTreeTable::new();
/// assert_eq!(bt.len(), 0);
/// bt.insert(0, "0");
/// bt.insert(1, "1");
/// bt.insert(2, "2");
/// assert_eq!(bt.len(), 3);
/// assert_eq!(bt.delete(&1), Some("1"));
/// assert_eq!(bt.len(), 2);
/// ```
#[derive(Debug, Clone, Default)]
pub struct BTreeTable<T, U> {
    tree: BTreeMap<T, U>,
}
impl<T, U> BTreeTable<T, U> {
    /// Tests whether or not the tree is empty.
    /// # Example
    /// ```
    /// use algods::data_structure::BTreeTable;
    /// let mut bt = BTreeTable::new();
    /// bt.insert(1,1);
    /// assert!(!bt.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    /// Gives the number of (key, value) pairs in the tree.
    /// # Example
    /// ```
    /// use algods::data_structure::BTreeTable;
    /// let bt = BTreeTable::<usize, usize>::new();
    /// assert_eq!(bt.len(),0);
    /// ```
    pub fn len(&self) -> usize {
        self.tree.len()
    }
}
impl<T: Ord, U> BTreeTable<T, U> {
    /// Creates an empty tree instance.
    /// # Example
    /// ```
    /// use algods::data_structure::BTreeTable;
    /// let bt = BTreeTable::<usize, isize>::new();
    /// assert_eq!(bt.len(), 0);
    /// ```
    pub fn new() -> Self {
        Self {
            tree: BTreeMap::<T, U>::new(),
        }
    }
    /// Creates a new tree with an initial (key, value) pair.
    /// # Example
    /// ```
    /// use algods::data_structure::BTreeTable;
    /// let bt = BTreeTable::init("btree", 0);
    /// assert_eq!(bt.len(), 1);
    /// ```
    pub fn init(key: T, value: U) -> Self {
        let mut tree = Self::new();
        tree.insert(key, value);
        tree
    }
    /// Tests whether or not the tree contains a given key.
    /// # Example
    /// ```
    /// use algods::data_structure::BTreeTable;
    /// let bt = BTreeTable::init("btree", "one");
    /// assert!(bt.contains(&"btree"));
    /// ```
    pub fn contains(&self, key: &T) -> bool {
        self.tree.get(key).is_some()
    }
    /// Returns a reference of the value associated to a key if any exists in the tree.
    /// Returns `None` otherwise.
    /// # Example
    /// ```
    /// use algods::data_structure::BTreeTable;
    /// let bt = BTreeTable::init("btree", "one");
    /// assert_eq!(bt.get(&"no btree"), None);
    /// assert_eq!(bt.get(&"btree"), Some(&"one"));
    /// ```    
    pub fn get(&self, key: &T) -> Option<&U> {
        self.tree.get(key)
    }
    /// Inserts a (key, value) pair in the tree.
    /// # Example
    /// ```
    /// use algods::data_structure::BTreeTable;
    /// let mut bt = BTreeTable::<isize, usize>::new();
    /// bt.insert(-1, 2);
    /// bt.insert(-2, 3);
    /// assert_eq!(bt.get(&-2), Some(&3));
    /// assert_eq!(bt.len(), 2);
    /// ```
    pub fn insert(&mut self, key: T, value: U) {
        self.tree.insert(key, value);
    }
    ///
    /// Removes a key from the tree, returning the value assiociated if any.
    /// Otherwise it returns `None`.
    /// # Example
    /// ```
    /// use algods::data_structure::BTreeTable;
    /// let mut bt = BTreeTable::init(1, 2);
    /// assert_eq!(bt.delete(&1), Some(2));
    /// assert_eq!(bt.delete(&10), None);
    /// ```
    pub fn delete(&mut self, key: &T) -> Option<U> {
        self.tree.remove(key)
    }
}
impl<T: Ord, U: Ord> BTreeTable<T, U> {
    /// Returns for the largest key in the tree strictly inferior.
    /// # Example
    /// ```
    /// use algods::data_structure::BTreeTable;
    /// let mut bt = BTreeTable::<isize, usize>::init(1, 0);
    /// bt.insert(-1, 2);
    /// bt.insert(-2, 3);
    /// assert_eq!(bt.strict_floor(&1), Some(&-1));
    /// assert_eq!(bt.strict_floor(&-2), None);
    pub fn strict_floor(&self, key: &T) -> Option<&T> {
        // the largest key in the tree map, strictly inferior to key
        let res = self.tree.range(..key).max();
        if let Some(item) = res {
            Some(item.0)
        } else {
            None
        }
    }
    /// Returns the smallest key larger or equal to the given key.
    /// # Example
    /// ```
    /// use algods::data_structure::BTreeTable;
    /// let mut bt = BTreeTable::<isize, usize>::init(1, 0);
    /// bt.insert(-1, 2);
    /// bt.insert(-2, 3);
    /// assert_eq!(bt.ceil(&1), Some(&1));
    /// assert_eq!(bt.ceil(&-1), Some(&-1));
    /// assert_eq!(bt.ceil(&2), None);
    /// ```
    pub fn ceil(&self, key: &T) -> Option<&T> {
        // the smallest key in the tree map larger ot equal to key
        let res = self.tree.range(key..).min();
        if let Some(item) = res {
            Some(item.0)
        } else {
            None
        }
    }
    /// Returns the list of keys in the tree that are between two keys (low included, high excluded).
    /// # Example
    /// ```
    /// use algods::data_structure::BTreeTable;
    /// let mut bt = BTreeTable::<isize, usize>::init(1, 0);
    /// bt.insert(-1, 2);
    /// bt.insert(-2, 2);
    /// bt.insert(-3, 3);
    /// assert_eq!(bt.range_search(&-2, &1), vec![&-2, &-1]);
    /// ```
    pub fn range_search(&self, low: &T, high: &T) -> Vec<&T> {
        // returns the keys between low (included) and high (excluded)
        self.tree
            .range(low..high)
            .into_iter()
            .map(|item| item.0)
            .collect::<Vec<&T>>()
    }
    /// Returns the number of keys in the tree that are between two keys (low included, high excluded).
    /// # Example
    /// ```
    /// use algods::data_structure::BTreeTable;
    /// let mut bt = BTreeTable::<isize, usize>::init(1, 0);
    /// bt.insert(-1, 2);
    /// bt.insert(-2, 2);
    /// bt.insert(-3, 3);
    /// assert_eq!(bt.range_count(&-3, &-1), 2);
    /// ```
    pub fn range_count(&self, low: &T, high: &T) -> usize {
        // counts the keys between low (included) and high (excluded)
        self.range_search(low, high).len()
    }
}

#[derive(Clone, Debug, PartialEq)]
struct Node<T, U> {
    key: T,
    value: U,
    left: Option<Box<Node<T, U>>>,
    right: Option<Box<Node<T, U>>>,
}
impl<T, U> Node<T, U> {
    pub fn init(_key: T, _value: U) -> Self {
        Self {
            key: _key,
            value: _value,
            left: None,
            right: None,
        }
    }
}

/// Implementation of a binary search tree
/// # Example
/// ```
/// use algods::data_structure::BSearchTree;
/// let mut bt = BSearchTree::new();
/// bt.insert(0,"1");
/// bt.insert(1,"2");
/// bt.insert(2,"3");
/// assert_eq!(bt.len(), 3);
/// assert!(bt.contains(&0));
/// assert_eq!(bt.get(&2), Some(&"3"));
/// ```
#[derive(Debug, Clone)]
pub struct BSearchTree<T, U> {
    root: Option<Box<Node<T, U>>>,
    len: usize,
}
impl<T, U> Default for BSearchTree<T, U> {
    fn default() -> Self {
        Self::new()
    }
}
impl<T, U> BSearchTree<T, U> {
    /// Creates an empty tree instance.
    /// # Example
    /// ```
    /// use algods::data_structure::BSearchTree;
    /// let bt = BSearchTree::<usize, isize>::new();
    /// assert_eq!(bt.len(), 0);
    /// ```
    pub fn new() -> Self {
        Self { root: None, len: 0 }
    }
    /// Creates a new tree with an initial (key, value) pair.
    /// # Example
    /// ```
    /// use algods::data_structure::BSearchTree;
    /// let bt = BSearchTree::init("btree", 0);
    /// assert_eq!(bt.len(), 1);
    /// ```
    pub fn init(key: T, value: U) -> Self {
        Self {
            root: Some(Box::new(Node::init(key, value))),
            len: 1,
        }
    }
    /// Gives the number of (key, value) pairs in the tree.
    /// # Example
    /// ```
    /// use algods::data_structure::BSearchTree;
    /// let bt = BSearchTree::<usize, usize>::new();
    /// assert_eq!(bt.len(),0);
    /// ```
    pub fn len(&self) -> usize {
        self.len
    }
    /// Tests whether or not the tree is empty.
    /// # Example
    /// ```
    /// use algods::data_structure::BSearchTree;
    /// let mut bt = BSearchTree::new();
    /// bt.insert(1,1);
    /// assert!(!bt.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
impl<T: Eq + Ord, U: Eq> BSearchTree<T, U> {
    /// Tests whether or not the tree contains a given key.
    /// # Example
    /// ```
    /// use algods::data_structure::BSearchTree;
    /// let bt = BSearchTree::init("btree", "one");
    /// assert!(bt.contains(&"btree"));
    /// ```
    pub fn contains(&self, key: &T) -> bool {
        self.get(key).is_some()
    }
    /// Returns a reference of the value associated to a key if any exists in the tree.
    /// Returns `None` otherwise.
    /// # Example
    /// ```
    /// use algods::data_structure::BSearchTree;
    /// let bt = BSearchTree::init("btree", "one");
    /// assert_eq!(bt.get(&"no btree"), None);
    /// assert_eq!(bt.get(&"btree"), Some(&"one"));
    /// ```
    pub fn get(&self, key: &T) -> Option<&U> {
        // gets the value associated to key if key is in
        // the tree, otherwise returns None,
        // run time complexity on average O(log(N)), O(N) guaranteed (unbalanced tree)
        let mut node = &self.root;
        while node.is_some() {
            let temp_node = node.as_ref().unwrap();
            match key.cmp(&temp_node.key) {
                Ordering::Less => node = &temp_node.left,
                Ordering::Greater => node = &temp_node.right,
                Ordering::Equal => return Some(&temp_node.value),
            }
        }
        None
    }
}
impl<T: Ord, U> BSearchTree<T, U> {
    fn put(node: &mut Option<Box<Node<T, U>>>, key: T, value: U) -> Option<&U> {
        match node {
            None => *node = Some(Box::new(Node::init(key, value))),
            Some(ref mut nod) => match key.cmp(&nod.key) {
                Ordering::Less => {
                    return Self::put(&mut nod.left, key, value);
                }
                Ordering::Greater => {
                    return Self::put(&mut nod.right, key, value);
                }
                Ordering::Equal => {
                    nod.value = value;
                    return Some(&nod.value);
                }
            },
        }
        None
    }
    /// Inserts a (key, value) pair in the tree.
    /// # Example
    /// ```
    /// use algods::data_structure::BSearchTree;
    /// let mut bt = BSearchTree::<isize, usize>::new();
    /// bt.insert(-1, 2);
    /// bt.insert(-2, 3);
    /// bt.insert(-1, 4);
    /// assert_eq!(bt.len(), 2);
    /// assert_eq!(bt.get(&-2), Some(&3));
    /// ```
    pub fn insert(&mut self, key: T, value: U) {
        if Self::put(&mut self.root, key, value).is_none() {
            self.len += 1;
        }
    }
}
impl<T: Eq + Ord, U: Ord> BSearchTree<T, U> {
    /// Returns the smallest key in the tree.
    /// # Example
    /// ```
    /// use algods::data_structure::BSearchTree;
    /// let mut bt = BSearchTree::<isize, usize>::init(1, 0);
    /// bt.insert(-1, 2);
    /// assert_eq!(bt.min(), Some(&-1));
    /// ```
    pub fn min(&self) -> Option<&T> {
        // finds the minimum key
        let mut node = &self.root;
        let mut result = None;
        while node.is_some() {
            // go to the left as long as you do not encounter
            // a None Node
            let temp_node = node.as_ref().unwrap();
            result = Some(&temp_node.key);
            node = &temp_node.left;
        }
        result
    }
    /// Returns the largest key in the tree.
    /// # Example
    /// ```
    /// use algods::data_structure::BSearchTree;
    /// let mut bt = BSearchTree::<isize, usize>::init(0, 0);
    /// bt.insert(-1, 2);
    /// assert_eq!(bt.max(), Some(&0));
    /// ```
    pub fn max(&self) -> Option<&T> {
        // finds the maximum key
        let mut node = &self.root;
        let mut result = None;
        while node.is_some() {
            // go to the right as long as you do not encounter
            // a None Node
            let temp_node = node.as_ref().unwrap();
            result = Some(&temp_node.key);
            node = &temp_node.right;
        }
        result
    }
    fn recursive_floor<'a>(
        node: &'a Option<Box<Node<T, U>>>,
        key: &T,
    ) -> &'a Option<Box<Node<T, U>>> {
        if node.is_none() {
            return &None;
        }
        let current_node = node.as_ref().unwrap();
        if key == &current_node.key {
            return node;
        }
        if key < &current_node.key {
            return Self::recursive_floor(&current_node.left, key);
        }
        let temp_node = Self::recursive_floor(&current_node.right, key);
        if temp_node.is_some() {
            temp_node
        } else {
            node
        }
    }
    /// Returns for the largest key in the tree smaller or equal to the input key.
    /// # Example
    /// ```
    /// use algods::data_structure::BSearchTree;
    /// let mut bt = BSearchTree::<isize, usize>::init(1, 0);
    /// bt.insert(-1, 2);
    /// bt.insert(-2, 3);
    /// assert_eq!(bt.floor(&1), Some(&1));
    /// assert_eq!(bt.floor(&0), Some(&-1));
    /// ```
    pub fn floor(&self, key: &T) -> Option<&T> {
        // the largest key in the tree smaller or equal to key
        // run time complexity O(log(N)) on average, O(N) (guaranteed)
        let node = Self::recursive_floor(&self.root, key);
        if node.is_none() {
            return None;
        }
        return Some(&node.as_ref().unwrap().key);
    }
}

/// Implementation of a tree map based on an ordered `Vec`.
/// # Example
/// ```
/// use algods::data_structure::OrdVecTable;
/// let mut table = OrdVecTable::new();
/// table.insert(0,"1");
/// table.insert(1,"2");
/// table.insert(2,"3");
/// assert_eq!(table.len(), 3);
/// assert!(table.contains(&0));
/// assert_eq!(table.get(&2), Some(&"3"));
// ###########################################
#[derive(Default, Clone, Debug)]
pub struct OrdVecTable<T, U> {
    // collection of key-value pair (no duplicate keys)
    vec: Vec<Pair<T, Option<U>>>,
}
impl<T, U> OrdVecTable<T, U> {
    /// Creates an empty tree instance.
    /// # Example
    /// ```
    /// use algods::data_structure::OrdVecTable;
    /// let tree = OrdVecTable::<usize, isize>::new();
    /// assert_eq!(tree.len(), 0);
    /// ```
    pub fn new() -> Self {
        Self { vec: Vec::new() }
    }
    /// Creates a new tree with an initial (key, value) pair.
    /// # Example
    /// ```
    /// use algods::data_structure::OrdVecTable;
    /// let table = OrdVecTable::init("table", 0);
    /// assert_eq!(table.len(), 1);
    /// ```
    pub fn init(key: T, value: U) -> Self {
        let mut symbol_table = Self::new();
        symbol_table.vec.push(Pair::init(key, Some(value)));
        symbol_table
    }
    /// Gives the number of (key, value) pairs in the tree.
    /// # Example
    /// ```
    /// use algods::data_structure::OrdVecTable;
    /// let table = OrdVecTable::<usize, usize>::new();
    /// assert_eq!(table.len(), 0);
    /// ```
    pub fn len(&self) -> usize {
        self.vec.len()
    }
    /// Tests whether or not the tree is empty.
    /// # Example
    /// ```
    /// use algods::data_structure::OrdVecTable;
    /// let mut table = OrdVecTable::new();
    /// table.insert(1,1);
    /// assert!(!table.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    /// Returns the smallest key in the tree.
    /// # Example
    /// ```
    /// use algods::data_structure::OrdVecTable;
    /// let mut table = OrdVecTable::<isize, usize>::init(1, 0);
    /// table.insert(-1, 2);
    /// assert_eq!(table.min(), Some(&-1));
    /// ```
    pub fn min(&self) -> Option<&T> {
        // smallest key O(1)
        if self.is_empty() {
            None
        } else {
            Some(self.vec[0].first())
        }
    }
    /// Returns the largest key in the tree.
    /// # Example
    /// ```
    /// use algods::data_structure::OrdVecTable;
    /// let mut table = OrdVecTable::<isize, usize>::init(1, 0);
    /// table.insert(-1, 2);
    /// assert_eq!(table.max(), Some(&1));
    /// ```
    pub fn max(&self) -> Option<&T> {
        // largest key O(1)
        if self.vec.is_empty() {
            None
        } else {
            Some(self.vec[self.vec.len() - 1].first())
        }
    }
}
impl<T: Ord + Clone, U: Eq> OrdVecTable<T, U> {
    /// Tests whether or not the tree contains a given key.
    /// # Example
    /// ```
    /// use algods::data_structure::OrdVecTable;
    /// let table = OrdVecTable::init("table", "one");
    /// assert!(table.contains(&"table"));
    /// ```
    pub fn contains(&self, key: &T) -> bool {
        // run time complexity O(log(N))
        self.get(key).is_some()
    }
    /// Returns a reference of the value associated to a key if any exists in the tree.
    /// Returns `None` otherwise.
    /// # Example
    /// ```
    /// use algods::data_structure::OrdVecTable;
    /// let table = OrdVecTable::init("tree", "one");
    /// assert_eq!(table.get(&"no tree"), None);
    /// assert_eq!(table.get(&"tree"), Some(&"one"));
    /// ```
    pub fn get(&self, key: &T) -> Option<&U> {
        // run time complexity O(log(N))
        if let Ok(index) = self.vec.binary_search(&Pair::init(key.clone(), None)) {
            return self.vec[index].second().as_ref();
        } else {
            None
        }
    }
    /// Returns for the largest key in the tree smaller or equal to the input key.
    /// # Example
    /// ```
    /// use algods::data_structure::OrdVecTable;
    /// let mut table = OrdVecTable::<isize, usize>::init(1, 0);
    /// table.insert(-1, 2);
    /// table.insert(-2, 3);
    /// assert_eq!(table.floor(&1), Some(&1));
    /// assert_eq!(table.floor(&0), Some(&-1));
    /// ```
    pub fn floor(&self, key: &T) -> Option<&T> {
        // largest key smaller or equal to key O(log(N))
        if self.is_empty() {
            None
        } else {
            let index = self.vec.binary_search(&Pair::init(key.clone(), None));
            match index {
                Ok(ind) => Some(self.vec[ind].first()),
                Err(ind) => {
                    if ind > 0 {
                        Some(self.vec[ind - 1].first())
                    } else {
                        // all keys in the table are > keys
                        None
                    }
                }
            }
        }
    }
    /// Returns for the smallest key in the tree larger or equal to the input key.
    /// # Example
    /// ```
    /// use algods::data_structure::OrdVecTable;
    /// let mut table = OrdVecTable::<isize, usize>::init(1, 0);
    /// table.insert(-1, 2);
    /// table.insert(-2, 3);
    /// assert_eq!(table.ceil(&1), Some(&1));
    /// assert_eq!(table.ceil(&2), None);
    /// assert_eq!(table.ceil(&-3), Some(&-2));
    /// ```
    pub fn ceil(&self, key: &T) -> Option<&T> {
        // smallest key larger or equal to key, O(log(N))
        if self.is_empty() {
            None
        } else {
            let index = self.vec.binary_search(&Pair::init(key.clone(), None));
            match index {
                Ok(ind) => Some(self.vec[ind].first()),
                Err(ind) => {
                    if ind < self.vec.len() - 1 && ind > 0 {
                        Some(self.vec[ind + 1].first())
                    } else if ind == 0 {
                        // key is < to all keys
                        Some(self.vec[0].first())
                    } else {
                        // all keys in the table are < key
                        None
                    }
                }
            }
        }
    }
}
impl<T: Ord + Clone, U: Eq + Clone> OrdVecTable<T, U> {
    fn put(&mut self, key: T, value: Option<U>) -> Option<U> {
        // run time complexity O(N) due to insertion
        let candidate = Pair::init(key.clone(), None);
        let index = self.vec.binary_search(&candidate);
        match index {
            Ok(ind) => {
                // key is found
                let temp_val = self.vec[ind].second().as_ref().cloned();
                let mut_val = self.vec[ind].second_mut();
                *mut_val = value;
                temp_val
            }
            Err(ind) => {
                // index where to insert key to keep self.vec sorted
                if ind < self.vec.len() {
                    self.vec.insert(ind, Pair::init(key, value));
                } else {
                    self.vec.push(Pair::init(key, value))
                }
                None
            }
        }
    }
    /// Inserts a (key, value) pair in the tree.
    /// # Example
    /// ```
    /// use algods::data_structure::OrdVecTable;
    /// let mut table = OrdVecTable::<isize, usize>::new();
    /// table.insert(-1, 2);
    /// table.insert(-2, 3);
    /// table.insert(-1, 4);
    /// assert_eq!(table.len(), 2);
    /// assert_eq!(table.get(&-2), Some(&3));
    /// ```
    pub fn insert(&mut self, key: T, value: U) {
        // run time complexity O(N)
        self.put(key, Some(value));
    }
    /// Deletes a key in the tree using a lazy implementation:
    /// meaning that it replaces the value of the key by `None` if any.
    /// # Example
    /// ```
    /// use algods::data_structure::OrdVecTable;
    /// let mut table = OrdVecTable::<isize, usize>::new();
    /// table.insert(-1, 2);
    /// table.insert(-2, 3);
    /// table.insert(-1, 4);
    /// assert_eq!(table.delete(&-1), Some(4));
    /// assert_eq!(table.delete(&0), None);
    /// ```
    pub fn delete(&mut self, key: &T) -> Option<U> {
        // run time complexity O(N)
        self.put(key.clone(), None) // lazy implementation
    }
}
#[derive(Default, Clone, Debug)]
struct Pair<T, U> {
    tuple: (T, U),
}
impl<T, U> Pair<T, U> {
    pub fn init(key: T, value: U) -> Self {
        Self {
            tuple: (key, value),
        }
    }
    pub fn first(&self) -> &T {
        &self.tuple.0
    }

    pub fn second(&self) -> &U {
        &self.tuple.1
    }
    pub fn first_mut(&mut self) -> &mut T {
        &mut self.tuple.0
    }
    pub fn second_mut(&mut self) -> &mut U {
        &mut self.tuple.1
    }
}
impl<T: Ord, U> Ord for Pair<T, U> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.tuple.0.cmp(&other.tuple.0)
    }
}
impl<T: Ord, U> PartialOrd for Pair<T, U> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.tuple.0.cmp(&other.tuple.0))
    }
}
impl<T: Ord, U> Eq for Pair<T, U> {}
impl<T: Ord, U> PartialEq for Pair<T, U> {
    fn eq(&self, other: &Self) -> bool {
        self.tuple.0 == other.tuple.0
    }
}

// ###############################################
/// Implementation of a tree map based on an unordered `Vec`.
/// # Example
/// ```
/// use algods::data_structure::UnordVecTable;
/// let mut table = UnordVecTable::new();
/// table.insert(0,"1");
/// table.insert(1,"2");
/// table.insert(2,"3");
/// assert_eq!(table.len(), 3);
/// assert!(table.contains(&0));
/// assert_eq!(table.get(&2), Some(&"3"));
#[derive(Default, Clone, Debug)]
pub struct UnordVecTable<T, U> {
    // collection of key-value pair (no duplicate keys)
    vec: Vec<(T, Option<U>)>,
}
impl<T, U> UnordVecTable<T, U> {
    /// Creates an empty tree instance.
    /// # Example
    /// ```
    /// use algods::data_structure::UnordVecTable;
    /// let tree = UnordVecTable::<usize, isize>::new();
    /// assert_eq!(tree.len(), 0);
    /// ```
    pub fn new() -> Self {
        Self { vec: Vec::new() }
    }
    /// Creates a new tree with an initial (key, value) pair.
    /// # Example
    /// ```
    /// use algods::data_structure::UnordVecTable;
    /// let table = UnordVecTable::init("table", 0);
    /// assert_eq!(table.len(), 1);
    /// ```
    pub fn init(key: T, value: U) -> Self {
        let mut symbol_table = Self::new();
        symbol_table.vec.push((key, Some(value)));
        symbol_table
    }
    /// Gives the number of (key, value) pairs in the tree.
    /// # Example
    /// ```
    /// use algods::data_structure::UnordVecTable;
    /// let table = UnordVecTable::<usize, usize>::new();
    /// assert_eq!(table.len(), 0);
    /// ```
    pub fn len(&self) -> usize {
        self.vec.len()
    }
    /// Tests whether or not the tree is empty.
    /// # Example
    /// ```
    /// use algods::data_structure::UnordVecTable;
    /// let mut table = UnordVecTable::new();
    /// table.insert(1,1);
    /// assert!(!table.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
impl<T: Eq, U: Eq> UnordVecTable<T, U> {
    /// Tests whether or not the tree contains a given key.
    /// # Example
    /// ```
    /// use algods::data_structure::UnordVecTable;
    /// let table = UnordVecTable::init("table", "one");
    /// assert!(table.contains(&"table"));
    /// ```
    pub fn contains(&self, key: &T) -> bool {
        // run time complexity O(N)
        self.get(key).is_some()
        // self.list.iter().any(|e| e.0 == key)
    }
}
impl<T: Eq, U> UnordVecTable<T, U> {
    /// Returns a reference of the value associated to a key if any exists in the tree.
    /// Returns `None` otherwise.
    /// # Example
    /// ```
    /// use algods::data_structure::UnordVecTable;
    /// let table = UnordVecTable::init("tree", "one");
    /// assert_eq!(table.get(&"no tree"), None);
    /// assert_eq!(table.get(&"tree"), Some(&"one"));
    /// ```
    pub fn get(&self, key: &T) -> Option<&U> {
        // run time complexity O(N)
        for k in 0..self.vec.len() {
            if &self.vec[k].0 == key {
                return self.vec[k].1.as_ref();
            }
        }
        None
    }
}
impl<T: Eq, U: Clone> UnordVecTable<T, U> {
    fn put(&mut self, key: T, value: Option<U>) -> Option<U> {
        // run time complexity O(N)
        let mut k = 0;
        let mut val = None;
        let length = self.vec.len();
        while k < length {
            if self.vec[k].0 == key {
                val = self.vec[k].1.clone();
                self.vec[k].1 = value.clone();
                break;
            }
            k += 1;
        }
        if self.is_empty() || (k == length && value.is_some()) {
            self.vec.push((key, value));
        }
        val
    }
    /// Inserts a (key, value) pair in the tree.
    /// # Example
    /// ```
    /// use algods::data_structure::UnordVecTable;
    /// let mut table = UnordVecTable::<isize, usize>::new();
    /// table.insert(-1, 2);
    /// table.insert(-2, 3);
    /// table.insert(-1, 4);
    /// assert_eq!(table.len(), 2);
    /// assert_eq!(table.get(&-1), Some(&4));
    /// ```
    pub fn insert(&mut self, key: T, value: U) {
        // run time complexity O(N)
        self.put(key, Some(value));
    }
}
impl<T: Eq + Clone, U: Clone> UnordVecTable<T, U> {
    /// Deletes a key in the tree using a lazy implementation:
    /// meaning that it replaces the value of the key by `None` if any.
    /// # Example
    /// ```
    /// use algods::data_structure::UnordVecTable;
    /// let mut table = UnordVecTable::<isize, usize>::new();
    /// table.insert(-1, 2);
    /// table.insert(-2, 3);
    /// table.insert(-1, 4);
    /// assert_eq!(table.delete(&-1), Some(4));
    /// assert_eq!(table.delete(&0), None);
    /// assert_eq!(table.len(), 2);
    /// ```
    pub fn delete(&mut self, key: &T) -> Option<U> {
        // run time complexity O(N)
        self.put(key.clone(), None) // lazy implementation
    }
}
