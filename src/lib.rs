pub mod heap;

pub use heap::{BoundedBinaryHeap, BoundedBinaryHeaper, heapify, verify_heap};

#[cfg(test)]
mod tests {
    use heap::{BoundedBinaryHeap, BoundedBinaryHeaper, heapify, verify_heap};
    #[test]
    fn test_empty_heap() {
        let h: BoundedBinaryHeap<i32> = BoundedBinaryHeap::new(3);
        assert_eq!(h.capacity(), 3);
        assert_eq!(h.len(), 0);
        assert!(h.is_empty());
    }

    #[test]
    fn test_heap() {
        let mut h = BoundedBinaryHeap::new(3);
        h.push(10);
        h.push(9);
        h.push(8);
        h.push(7);
        h.push(6);
        h.push(5);
        assert_eq!(h.len(), 3);
        assert_eq!(h.capacity(), 3);
        assert_eq!(*h.peek().unwrap(), 8);
        assert_eq!(h.pop().unwrap(), 8);
        assert_eq!(h.pop().unwrap(), 9);
        assert_eq!(h.pop().unwrap(), 10);
        assert_eq!(h.pop(), None);
        assert!(h.is_empty());
    }

    #[test]
    fn test_heap_heapify() {
        let mut h = BoundedBinaryHeap::from(&[7,9,8]);
        assert_eq!(h.capacity(), 3);
        assert_eq!(h.len(), 3);
        assert!(!h.is_empty());
        h.push(10);
        h.push(6);
        h.push(5);
        assert_eq!(h.len(), 3);
        assert_eq!(h.capacity(), 3);
        assert_eq!(*h.peek().unwrap(), 8);
        assert_eq!(h.pop().unwrap(), 8);
        assert_eq!(h.pop().unwrap(), 9);
        assert_eq!(h.pop().unwrap(), 10);
        assert_eq!(h.pop(), None);
        assert!(h.is_empty());
    }

    #[test]
    fn test_heap_heapify_with_capacity() {
        let mut h = BoundedBinaryHeap::from_slice_with_capacity(&[7,9,8], 5);
        assert_eq!(h.capacity(), 5);
        assert_eq!(h.len(), 3);
        assert!(!h.is_empty());
        h.push(6);
        h.push(5);
        h.push(10);
        assert_eq!(h.len(), 5);
        assert_eq!(h.capacity(), 5);
        assert_eq!(*h.peek().unwrap(), 6);
        assert_eq!(h.pop().unwrap(), 6);
        assert_eq!(h.pop().unwrap(), 7);
        assert_eq!(h.pop().unwrap(), 8);
        assert_eq!(h.pop().unwrap(), 9);
        assert_eq!(h.pop().unwrap(), 10);
        assert_eq!(h.pop(), None);
        assert!(h.is_empty());
    }

    #[test]
    fn test_heaper() {
        let mut v=[10, 9, 8];
        let mut h = BoundedBinaryHeaper::from(&mut v);
        h.push(7);
        h.push(6);
        h.push(5);
        assert_eq!(h.len(), 3);
        assert_eq!(h.capacity(), 3);
        assert_eq!(*h.peek().unwrap(), 8);
        assert_eq!(*h.pop().unwrap(), 8);
        assert_eq!(*h.pop().unwrap(), 9);
        assert_eq!(*h.pop().unwrap(), 10);
        assert_eq!(h.pop(), None);
        assert!(h.is_empty());
    }

    #[test]
    fn test_heapify() {
        let mut v=[10, 12, 9, 8, 7, 6, 5, 11, 3];
        heapify(&mut v);
        assert_eq!(v[0], 3);
        assert!(verify_heap(&v));
    }

}
