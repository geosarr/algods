#[cfg(test)]
mod unit_test;

use std::collections::LinkedList;
// use std::mem::replace;

/// Implementation of the First In First Out concept (namely a queue),
/// with the standard library
/// # Examples
/// ```
/// use algods::data_structure::Queue;
/// let mut queue = Queue::new();
/// assert_eq!(queue.len(), 0);
/// queue.enqueue(0);
/// queue.enqueue(1);
/// queue.enqueue(2);
/// assert_eq!(queue.len(), 3);
/// assert_eq!(queue.dequeue(), Some(0));
/// assert_eq!(queue.dequeue(), Some(1));
/// assert_eq!(queue.len(), 1);
/// ```
#[derive(Default, Clone, Debug)]
pub struct Queue<T> {
    list: LinkedList<T>,
}

impl<T> Queue<T> {
    /// Creates an empty queue instance.
    /// # Example
    /// ```
    /// use algods::data_structure::Queue;
    /// let queue = Queue::<usize>::new();
    /// assert_eq!(queue.len(), 0);
    /// ```
    pub fn new() -> Self {
        Self {
            list: LinkedList::new(),
        }
    }

    /// Creates a new queue with an initial object.
    /// # Example
    /// ```
    /// use algods::data_structure::Queue;
    /// let queue = Queue::init("queue");
    /// assert_eq!(queue.len(), 1);
    /// ```
    pub fn init(s: T) -> Self {
        let mut res = Self {
            list: LinkedList::new(),
        };
        res.enqueue(s);
        res
    }

    /// Tests whether or not the queue is empty.
    /// # Example
    /// ```
    /// use algods::data_structure::Queue;
    /// let queue = Queue::<usize>::new();
    /// assert!(queue.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.list.is_empty()
    }

    /// Gives the number of objects in the queue.
    /// # Example
    /// ```
    /// use algods::data_structure::Queue;
    /// let queue = Queue::<isize>::new();
    /// assert_eq!(queue.len(),0);
    /// ```
    pub fn len(&self) -> usize {
        self.list.len()
    }

    /// Deletes and returns the first object in the queue, if any.
    /// Otherwise, it returns `None`.
    /// # Example
    /// ```
    /// use algods::data_structure::Queue;
    /// let mut queue = Queue::init(1);
    /// assert_eq!(queue.dequeue(), Some(1));
    /// ```
    pub fn dequeue(&mut self) -> Option<T> {
        self.list.pop_front()
    }

    /// Inserts an object at the end the queue.
    /// # Example
    /// ```
    /// use algods::data_structure::Queue;
    /// let mut queue = Queue::<isize>::new();
    /// queue.enqueue(-1);
    /// queue.enqueue(-2);
    /// assert_eq!(queue.dequeue(), Some(-1));
    /// ```
    pub fn enqueue(&mut self, element: T) {
        self.list.push_back(element)
    }
}
