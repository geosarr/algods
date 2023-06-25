#[cfg(test)]
mod unit_test;

use std::collections::LinkedList;

/// Implementation of deques with the standard library
/// # Examples
/// ```
/// use algods::data_structure::Deque;
/// let mut deque = Deque::new();
/// assert_eq!(deque.len(), 0);
/// deque.add_first(0);
/// deque.add_last(1);
/// deque.add_first(2);
/// assert_eq!(deque.len(), 3);
/// assert_eq!(deque.remove_last(), Some(1));
/// assert_eq!(deque.remove_first(), Some(2));
/// assert_eq!(deque.len(), 1);
/// ```
#[derive(Default, Clone, Debug)]
pub struct Deque<T> {
    list: LinkedList<T>,
}

impl<T> Deque<T> {
    /// Creates an empty deque instance.
    /// # Example
    /// ```
    /// use algods::data_structure::Deque;
    /// let deque = Deque::<usize>::new();
    /// assert_eq!(deque.len(), 0);
    /// ```
    pub fn new() -> Self {
        Self {
            list: LinkedList::new(),
        }
    }

    /// Creates a new deque with an initial object.
    /// # Example
    /// ```
    /// use algods::data_structure::Deque;
    /// let deque = Deque::init("deque");
    /// assert_eq!(deque.len(), 1);
    /// ```
    pub fn init(s: T) -> Self {
        let mut res = Self {
            list: LinkedList::new(),
        };
        res.add_first(s);
        res
    }

    /// Tests whether or not the deque is empty.
    /// # Example
    /// ```
    /// use algods::data_structure::Deque;
    /// let deque = Deque::<usize>::new();
    /// assert!(deque.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.list.is_empty()
    }

    /// Gives the number of objects in the deque.
    /// # Example
    /// ```
    /// use algods::data_structure::Deque;
    /// let deque = Deque::<isize>::new();
    /// assert_eq!(deque.len(),0);
    /// ```
    pub fn len(&self) -> usize {
        self.list.len()
    }

    /// Inserts an object at the beginning of the deque.
    /// # Example
    /// ```
    /// use algods::data_structure::Deque;
    /// let mut deque = Deque::<isize>::new();
    /// deque.add_first(-1);
    /// deque.add_first(-2);
    /// assert_eq!(deque.remove_first(), Some(-2));
    /// ```
    pub fn add_first(&mut self, elt: T) {
        self.list.push_front(elt)
    }

    /// Inserts an object at the end of the deque.
    /// # Example
    /// ```
    /// use algods::data_structure::Deque;
    /// let mut deque = Deque::<isize>::new();
    /// deque.add_last(-1);
    /// deque.add_last(-2);
    /// assert_eq!(deque.remove_first(), Some(-1));
    /// ```
    pub fn add_last(&mut self, elt: T) {
        self.list.push_back(elt)
    }

    /// Deletes and returns the first object in the deque, if any.
    /// Returns `None` otherwise.
    /// # Example
    /// ```
    /// use algods::data_structure::Deque;
    /// let mut deque = Deque::init(1);
    /// deque.add_last(0);
    /// deque.add_last(1);
    /// assert_eq!(deque.remove_first(), Some(1));
    /// ```
    pub fn remove_first(&mut self) -> Option<T> {
        self.list.pop_front()
    }

    /// Deletes and returns the last object in the deque, if any.
    /// Returns `None` otherwise.
    /// # Example
    /// ```
    /// use algods::data_structure::Deque;
    /// let mut deque = Deque::init(1);
    /// deque.add_last(0);
    /// deque.add_last(1);
    /// assert_eq!(deque.remove_last(), Some(1));
    /// ```
    pub fn remove_last(&mut self) -> Option<T> {
        self.list.pop_back()
    }
}
