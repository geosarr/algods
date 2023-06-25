#[cfg(test)]
mod unit_test;

use std::collections::LinkedList;
use std::mem::replace;

/// Implementation of an elementary node
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Node<T> {
    item: T,
    // Box helps avoid infinity memory allocation
    // in a recursive definition of a struct
    next: Option<Box<Node<T>>>,
}
impl<T> Node<T> {
    pub fn get_item(&self) -> &T {
        &self.item
    }
    pub fn get_mut_item(&mut self) -> &mut T {
        &mut self.item
    }
    pub fn get_next(&self) -> &Option<Box<Node<T>>> {
        &self.next
    }
    pub fn get_mut_next(&mut self) -> &mut Option<Box<Node<T>>> {
        &mut self.next
    }
}

/// Implementation of the First In Last Out concept, relatively from scratch
/// # Examples
/// ```
/// use algods::data_structure::Stack;
/// let mut stack = Stack::new();
/// assert_eq!(stack.len(), 0);
/// stack.push(0);
/// stack.push(1);
/// stack.push(2);
/// assert_eq!(stack.len(), 3);
/// assert_eq!(stack.pop(), Some(2));
/// assert_eq!(stack.len(), 2);
/// ```
#[derive(Debug, Default, Clone)]
pub struct Stack<T> {
    first: Option<Box<Node<T>>>,
    len: usize,
}

impl<T> Stack<T> {
    /// Creates an empty stack instance.
    /// # Example
    /// ```
    /// use algods::data_structure::Stack;
    /// let stack = Stack::<usize>::new();
    /// assert_eq!(stack.len(), 0);
    /// ```
    pub fn new() -> Self {
        // run time complexity O(1)
        Self {
            first: None,
            len: 0,
        }
    }

    /// Creates a new stack with an initial object.
    /// # Example
    /// ```
    /// use algods::data_structure::Stack;
    /// let stack = Stack::init("stack");
    /// assert_eq!(stack.len(), 1);
    /// ```
    pub fn init(s: T) -> Self {
        // run time complexity O(1)
        let node = Node {
            item: s,
            next: None,
        };
        Self {
            first: Some(Box::new(node)),
            len: 1,
        }
    }

    /// Gives an immutable reference to the first object to come out of the stack,
    /// i.e the last element inserted in the stack.
    /// # Example
    /// ```
    /// use algods::data_structure::Stack;
    /// let stack = Stack::init("0");
    /// assert_eq!(stack.get_first().as_ref().unwrap().get_item().clone(), "0");
    /// ```
    pub fn get_first(&self) -> &Option<Box<Node<T>>> {
        &self.first
    }

    /// Gives a mutable reference to the last element inserted in the stack.
    /// # Example
    /// ```
    /// use algods::data_structure::Stack;
    /// let mut stack = Stack::init("0");
    /// let mut mut_ref_first = stack.get_mut_first();
    /// if let Some(ref mut node) = mut_ref_first{
    ///     *(node.get_mut_item()) = "1";     
    ///  }
    /// assert_eq!(stack.get_first().as_ref().unwrap().get_item().clone(), "1");
    /// ```
    pub fn get_mut_first(&mut self) -> &mut Option<Box<Node<T>>> {
        &mut self.first
    }

    /// Tests whether or not the stack is empty.
    /// # Example
    /// ```
    /// use algods::data_structure::Stack;
    /// let stack = Stack::<usize>::new();
    /// assert!(stack.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        // run time complexity O(1)
        self.first.is_none()
    }

    /// Gives the number of objects in the stack.
    /// # Example
    /// ```
    /// use algods::data_structure::Stack;
    /// let stack = Stack::<isize>::new();
    /// assert_eq!(stack.len(),0);
    /// ```
    pub fn len(&self) -> usize {
        // run time complexity O(1)
        self.len
    }
}

impl<T: Clone> Stack<T> {
    /// Deletes and returns the last object in the stack, if any.
    /// # Panics
    /// When there is no element in the stack, it panics.
    /// # Example
    /// ```
    /// use algods::data_structure::Stack;
    /// let mut stack = Stack::init(1);
    /// assert_eq!(stack.pop(), Some(1));
    /// ```
    pub fn pop(&mut self) -> Option<T> {
        // run time complexity O(N) (due to cloning) ?
        // space complexity O(N) (due to cloning) ?
        match self.first {
            Some(ref node) => {
                let item = node.item.clone();
                self.first = node.next.clone();
                self.len -= 1;
                Some(item)
            }
            None => panic!("cannot pop, stack is empty"),
        }
    }

    /// Inserts an object into the stack.
    /// # Example
    /// ```
    /// use algods::data_structure::Stack;
    /// let mut stack = Stack::<isize>::new();
    /// stack.push(-1);
    /// stack.push(-2);
    /// assert_eq!(stack.pop(), Some(-2));
    /// ```
    pub fn push(&mut self, s: T) {
        // run time complexity O(1) (due to cloning) ?
        let oldfirst = self.first.clone();
        let new_node = Node {
            item: s,
            next: oldfirst,
        };
        self.first = Some(Box::new(new_node));
        self.len += 1;
    }
}

/// Implementation of stacks using the standard library
/// # Examples
/// ```
/// use algods::data_structure::ListStack;
/// let mut stack = ListStack::new();
/// assert_eq!(stack.len(), 0);
/// stack.push(0);
/// stack.push(1);
/// stack.push(2);
/// assert_eq!(stack.len(), 3);
/// assert_eq!(stack.pop(), Some(2));
/// assert_eq!(stack.len(), 2);
/// ```
#[derive(Debug, Clone)]
pub struct ListStack<T> {
    list: LinkedList<T>,
}
impl<T> Default for ListStack<T> {
    fn default() -> Self {
        Self::new()
    }
}
impl<T> ListStack<T> {
    /// Creates an empty stack instance.
    /// # Example
    /// ```
    /// use algods::data_structure::ListStack;
    /// let stack = ListStack::<usize>::new();
    /// assert_eq!(stack.len(), 0);
    /// ```
    pub fn new() -> Self {
        Self {
            list: LinkedList::new(),
        }
    }

    /// Creates a new stack with an initial object.
    /// # Example
    /// ```
    /// use algods::data_structure::ListStack;
    /// let stack = ListStack::init("stack");
    /// assert_eq!(stack.len(), 1);
    /// ```
    pub fn init(s: T) -> Self {
        let mut res = Self {
            list: LinkedList::new(),
        };
        res.push(s);
        res
    }

    /// Tests whether or not the stack is empty.
    /// # Example
    /// ```
    /// use algods::data_structure::ListStack;
    /// let stack = ListStack::<usize>::new();
    /// assert!(stack.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.list.is_empty()
    }

    /// Gives the number of objects in the stack.
    /// # Example
    /// ```
    /// use algods::data_structure::ListStack;
    /// let stack = ListStack::<isize>::new();
    /// assert_eq!(stack.len(),0);
    /// ```
    pub fn len(&self) -> usize {
        self.list.len()
    }

    /// Deletes and returns the last object in the stack, if any.
    /// Otherwise it returns `None`.
    /// # Example
    /// ```
    /// use algods::data_structure::ListStack;
    /// let mut stack = ListStack::init(1);
    /// assert_eq!(stack.pop(), Some(1));
    /// ```
    pub fn pop(&mut self) -> Option<T> {
        self.list.pop_back()
    }

    /// Inserts an object into the stack.
    /// # Example
    /// ```
    /// use algods::data_structure::ListStack;
    /// let mut stack = ListStack::<isize>::new();
    /// stack.push(-1);
    /// stack.push(-2);
    /// assert_eq!(stack.pop(), Some(-2));
    /// ```
    pub fn push(&mut self, element: T) {
        self.list.push_back(element)
    }
}

/// Implementation of stacks using a fixed size `Vec` with
/// capacity doubling when full and size halving when 25% full
/// # Examples
/// ```
/// use algods::data_structure::VecStack;
/// let mut stack = VecStack::<usize>::new();
/// assert_eq!(stack.len(), 0);
/// stack.push(0);
/// stack.push(1);
/// stack.push(2);
/// assert_eq!(stack.len(), 3);
/// assert_eq!(stack.pop(), Some(2));
/// assert_eq!(stack.len(), 2);
/// ```
#[derive(Debug, Clone)]
pub struct VecStack<T> {
    // Number of not None values in vec, i.e the number of objects
    n: usize,
    // Contains the objects
    vec: Vec<Option<T>>,
}
impl<T> Default for VecStack<T> {
    fn default() -> Self {
        Self::new()
    }
}
impl<T> VecStack<T> {
    /// Creates an empty stack instance.
    /// # Example
    /// ```
    /// use algods::data_structure::VecStack;
    /// let stack = VecStack::<usize>::new();
    /// assert_eq!(stack.len(), 0);
    /// ```
    pub fn new() -> Self {
        Self {
            n: 0,
            vec: vec![None],
        }
    }

    /// Creates a stack with an initial capacity.
    /// # Panics
    /// If `capacity = 0`, then it panics.
    /// # Example
    /// ```
    /// use algods::data_structure::VecStack;
    /// let stack = VecStack::<usize>::with_capacity(2);
    /// assert_eq!(stack.len(), 0);
    /// ```
    pub fn with_capacity(capacity: usize) -> Self {
        // run time complexity O(capacity)
        if capacity > 0 {
            let mut vector = Vec::with_capacity(capacity);
            for _ in 0..capacity {
                vector.push(None);
            }

            Self { n: 0, vec: vector }
        } else {
            panic!("capacity shoul be > 0");
        }
    }

    /// Tests whether or not the stack is empty.
    /// # Example
    /// ```
    /// use algods::data_structure::VecStack;
    /// let stack = VecStack::<usize>::new();
    /// assert!(stack.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        // run time complexity O(1)
        self.n == 0
    }

    /// Gives the number of objects in the stack.
    /// # Example
    /// ```
    /// use algods::data_structure::VecStack;
    /// let stack = VecStack::<isize>::new();
    /// assert_eq!(stack.len(),0);
    /// ```
    pub fn len(&self) -> usize {
        self.n
    }

    /// Inserts an object into the stack.
    /// # Example
    /// ```
    /// use algods::data_structure::VecStack;
    /// let mut stack = VecStack::<isize>::new();
    /// stack.push(-1);
    /// stack.push(-2);
    /// assert_eq!(stack.pop(), Some(-2));
    /// ```
    pub fn push(&mut self, obj: T) {
        // run time complexity O(1)
        if self.n < self.vec.len() {
            // The stack is not full yet
            // This works only if the stack has a capacity > 0
            // If the capacity is zero then panic
            let _ = replace(&mut self.vec[self.n], Some(obj));
            self.n += 1;
            if self.n == self.vec.len() {
                // double the stack to allow more capacity
                self.double();
            }
        } else {
            panic!("cannot push, stack is full or has capacity 0");
        }
    }

    /// Deletes and returns the last object in the stack, if any.
    /// # Panics
    /// When there is no element in the stack, it panics.
    /// # Example
    /// ```
    /// use algods::data_structure::VecStack;
    /// let mut stack = VecStack::new();
    /// stack.push(1);
    /// stack.push(0);
    /// assert_eq!(stack.pop(), Some(0));
    /// ```
    pub fn pop(&mut self) -> Option<T> {
        // run time complexity O(1)
        if self.n > 0 && self.n <= self.vec.len() {
            let elt = replace(&mut self.vec[self.n - 1], None);
            self.n -= 1;
            if self.n <= self.vec.len() / 4 {
                self.halve();
            }
            elt
        } else {
            panic!("cannot pop, stack is empty");
        }
    }

    fn double(&mut self) {
        // run time complexity O(N)
        // doubling the size of the stack
        let mut vector = Vec::with_capacity(self.vec.len());
        for _ in 0..self.vec.len() {
            vector.push(None);
        }
        self.vec.append(&mut vector);
    }

    fn halve(&mut self) {
        // run time complexity O(N)
        // halving the size of the stack
        self.vec.truncate(self.vec.len() / 2);
    }
}
