#[cfg(test)]
mod tests {
    use super::super::*;

    fn unbox<T>(value: Box<T>) -> T {
        *value
    }

    #[test]
    fn test_stack_init() {
        let string = "test".to_string();
        let stack = Stack::<String>::init(string.clone());
        assert_eq!(string, unbox(stack.first.unwrap()).item);
        assert_eq!(1, stack.len);
    }

    #[test]
    #[should_panic]
    fn test_stack_pop() {
        let string = "test".to_string();
        let mut stack = Stack::<String>::init(string.clone());
        assert_eq!(Some(string.clone()), stack.pop());
        assert_eq!(0, stack.len);
        stack.pop();
    }

    #[test]
    fn test_stack_push() {
        let string1 = "test".to_string();
        let string2 = "test2".to_string();
        let mut stack = Stack::<String>::new();
        assert_eq!(0, stack.len);
        stack.push(string1.clone());
        stack.push(string2.clone());
        assert_eq!(2, stack.len);
        assert_eq!(Some(string2.clone()), stack.pop());
        assert_eq!(Some(string1.clone()), stack.pop());
    }

    #[test]
    fn test_stack_is_empty() {
        let mut stack = Stack::<String>::new();
        assert!(stack.is_empty());
        stack.push("test".to_string());
        assert!(!stack.is_empty());
    }

    #[test]
    fn test_linked_list_stack_new() {
        let stack = ListStack::<usize>::new();
        assert!(stack.is_empty());
    }

    #[test]
    fn test_linked_list_stack_init() {
        let string = "test".to_string();
        let mut stack = ListStack::init(string.clone());
        assert_eq!(Some(string), stack.pop());
    }

    #[test]
    fn test_linked_list_stack_pop() {
        let num = -1isize;
        let mut stack = ListStack::init(num);
        assert_eq!(Some(num), stack.pop());
        assert_eq!(None, stack.pop());
    }

    #[test]
    fn test_linked_list_stack_push() {
        let string1 = "test".to_string();
        let string2 = "test2".to_string();
        let mut stack = ListStack::new();
        stack.push(string1.clone());
        stack.push(string2.clone());
        assert_eq!(Some(string2.clone()), stack.pop());
        assert_eq!(Some(string1.clone()), stack.pop());
    }

    #[test]
    fn test_vec_stack_with_capacity() {
        let n = 50;
        let stack = VecStack::<String>::with_capacity(n);
        let none: Option<String> = None;
        assert_eq!(stack.len(), 0);
        assert_eq!(false, stack.vec.iter().any(|e| *e != none));
    }

    #[test]
    #[should_panic]
    fn test_vec_stack_with_capacity_panic() {
        VecStack::<isize>::with_capacity(0);
    }

    #[test]
    fn test_vec_stack_pop() {
        let string = "test".to_string();
        let mut stack = VecStack::<String>::with_capacity(1);
        stack.push(string.clone());
        assert_eq!(Some(string), stack.pop());
    }

    #[test]
    #[should_panic]
    fn test_vec_stack_pop_panic() {
        let string = "test".to_string();
        let mut stack = VecStack::<String>::with_capacity(1);
        stack.push(string.clone());
        stack.pop();
        stack.pop();
    }

    #[test]
    fn test_vec_stack_push() {
        let string1 = "test".to_string();
        let string2 = "test2".to_string();
        let mut stack = VecStack::<String>::new();
        stack.push(string1.clone());
        stack.push(string2.clone());
        assert_eq!(Some(string2), stack.pop());
        assert_eq!(Some(string1), stack.pop());
    }

    #[test]
    fn test_vec_stack_is_empty() {
        let mut stack = VecStack::<usize>::new();
        assert!(stack.is_empty());
        assert_eq!(0, stack.len());
        stack.push(1);
        assert!(!stack.is_empty());
    }
    #[test]
    fn test_vec_stack_resize() {
        let mut stack = VecStack::<usize>::new();
        stack.push(1);
        assert!(stack.vec.len() == 2);
        stack.push(5);
        assert!(stack.vec.len() == 4);
        stack.pop();
        assert_eq!(stack.vec.len(), 2);
    }
}
