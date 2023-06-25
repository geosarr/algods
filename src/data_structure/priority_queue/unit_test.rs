#[cfg(test)]
mod tests {
    use super::super::*;
    use rand::Rng;

    #[test]
    fn test_binary_heap_priority_queue() {
        let len = 10000000;
        let mut rng = rand::thread_rng();
        let mut queue = BinaryHeapQueue::<isize>::with_capacity(len);
        assert!(queue.is_empty());
        for _ in 0..len {
            queue.insert(rng.gen::<isize>());
        }
        assert_eq!(queue.len(), len);
        assert_eq!(
            queue.heap.peek().unwrap().clone(),
            queue.heap.pop().unwrap()
        );
    }

    #[test]
    #[should_panic]
    fn test_init_priority_queue() {
        PriorityQueue::<String>::with_capacity(0, Orientation::Min);
    }

    #[test]
    fn test_min_priority_queue() {
        let len = 1000000;
        let mut rng = rand::thread_rng();
        let mut queue = PriorityQueue::<isize>::with_capacity(2, Orientation::Min);
        assert!(queue.is_empty());
        queue.insert(rng.gen::<isize>());
        assert_eq!(queue.len(), 1);
        queue.insert(rng.gen::<isize>());
        assert_eq!(queue.vec.len(), 6);
        for _ in 0..len - 2 {
            queue.insert(rng.gen::<isize>());
        }
        queue.delete();
        assert_eq!(queue.len(), len - 1);
        assert_eq!(queue.extremum(), queue.vec[1].as_ref());
        for k in 1..(queue.len() - 1) / 2 {
            assert!(queue.vec[k] <= queue.vec[2 * k] && queue.vec[k] <= queue.vec[2 * k + 1]);
        }
    }

    #[test]
    fn test_max_priority_queue() {
        let len = 1000000;
        let mut rng = rand::thread_rng();
        let mut queue = PriorityQueue::<isize>::with_capacity(2, Orientation::Max);
        assert!(queue.is_empty());
        queue.insert(rng.gen::<isize>());
        assert_eq!(queue.len(), 1);
        queue.insert(rng.gen::<isize>());
        assert_eq!(queue.vec.len(), 6);
        for _ in 0..len - 2 {
            queue.insert(rng.gen::<isize>());
        }
        queue.delete();
        assert_eq!(queue.len(), len - 1);
        assert_eq!(queue.extremum(), queue.vec[1].as_ref());
        for k in 1..(queue.len() - 1) / 2 {
            assert!(queue.vec[k] >= queue.vec[2 * k] && queue.vec[k] >= queue.vec[2 * k + 1]);
        }
    }
}
