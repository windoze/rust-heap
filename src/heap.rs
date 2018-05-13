use std::mem;

fn sift_up<T: PartialOrd>(heap: &mut [T]) {
    let mut i = heap.len();
    let mut j = i/2;
    while (j>0) && (heap[i-1]<heap[j-1]) {
        heap.swap(i-1, j-1);
        i = j;
        j = j/2;
    }
}

fn sift_down<T: PartialOrd>(heap: &mut [T], mut i: usize) {
    i = i+1;
    let mut j = i*2;
    let mut k = j+1;
    // find smaller child
    if (k<=heap.len()) && (heap[k-1]<heap[j-1]) {
        j = k;
    }
    while (j<=heap.len()) && (heap[j-1]<heap[i-1]) {
        heap.swap(i-1, j-1);
        i = j;
        j = i*2;
        k = j+1;
        if (k<=heap.len()) && (heap[k-1]<heap[j-1]) {
            j = k;
        }
    }
}

/// Make a slice into binary heap
/// ```
/// extern crate rust_heap;
/// use rust_heap::heap::heapify;
/// let mut v=[3,2,1];
/// heapify(&mut v);
/// assert_eq!(v[0], 1);
/// ```
pub fn heapify<T: PartialOrd>(heap: &mut [T]) {
    for i in (0..((heap.len()-1)/2)).rev() {
        sift_down(heap, i);
    }
}

/// Verify if a slice is a binary heap
/// ```
/// extern crate rust_heap;
/// use rust_heap::heap::verify_heap;
/// let mut v=[1,2,3];
/// assert!(verify_heap(&v));
/// let mut v=[3,2,1];
/// assert!(!verify_heap(&v));
/// ```
pub fn verify_heap<T:PartialOrd>(heap: &[T]) -> bool {
    for i in 0..((heap.len()-1)/2) {
        if heap[i]>heap[((i+1)*2-1)] || heap[i]>heap[((i+1)*2)] { return false; }
    }
    true
}

/// Heaper converts underlying slice into a binary heap with bound
pub struct BoundedBinaryHeaper<'a, T: 'a> {
    len: usize,
    heap: &'a mut [T],
}

impl<'a, T: 'a+PartialOrd> BoundedBinaryHeaper<'a, T>
{
    /// Create a new BoundedBinaryHeaper on top of a slice
    /// ```
    /// extern crate rust_heap;
    /// use rust_heap::heap::BoundedBinaryHeaper;
    /// let mut v=[5,4,3,2,1];
    /// let mut h = BoundedBinaryHeaper::from(&mut v);
    /// assert_eq!(h.capacity(), 5);
    /// assert_eq!(h.len(), 5);
    /// ```
    pub fn from(slice: &'a mut [T]) -> BoundedBinaryHeaper<'a, T> {
        heapify(slice);
        BoundedBinaryHeaper {
            len: slice.len(),
            heap: slice,
        }
    }

    /// Create a new BoundedBinaryHeaper on top of an slice and ignore all contents in it
    /// ```
    /// extern crate rust_heap;
    /// use rust_heap::heap::BoundedBinaryHeaper;
    /// let mut v=[5,4,3,2,1];
    /// let mut h = BoundedBinaryHeaper::from_empty_slice(&mut v);
    /// assert_eq!(h.capacity(), 5);
    /// assert_eq!(h.len(), 0);
    /// ```
    pub fn from_empty_slice(slice: &'a mut [T]) -> BoundedBinaryHeaper<'a, T> {
        BoundedBinaryHeaper {
            len: 0,
            heap: slice,
        }
    }

    /// Push a value into heap if value>=root and keep heap structure
    /// Overflow and return the smallest element when heap is full
    /// ```
    /// extern crate rust_heap;
    /// use rust_heap::heap::BoundedBinaryHeaper;
    /// let mut v=[3,2,1];
    /// let mut h = BoundedBinaryHeaper::from(&mut v);
    /// assert_eq!(h.push(4).unwrap(), 1);
    /// assert_eq!(h.len(), 3);
    /// ```
    pub fn push(&mut self, mut elem: T) -> Option<T> {
        if self.len<self.capacity() {
            self.heap[self.len]=elem;
            self.len+=1;
            let l=self.len;
            sift_up(&mut self.heap[0..l]);
            None
        } else if (self.len>0) && (elem>=self.heap[0]) {
            mem::swap(&mut elem, &mut self.heap[0]);
            sift_down(self.heap, 0);
            Some(elem)
        } else {
            Some(elem)
        }
    }

    /// Pop the smallest value from heap
    /// ```
    /// extern crate rust_heap;
    /// use rust_heap::heap::BoundedBinaryHeaper;
    /// let mut v=[3,2,1];
    /// let mut h = BoundedBinaryHeaper::from(&mut v);
    /// assert_eq!(*h.pop().unwrap(), 1);
    /// ```
    pub fn pop(&mut self) -> Option<&T> {
        if self.len>0 {
            self.heap.swap(0, self.len-1);
            self.len-=1;
            let l=self.len;
            sift_down(&mut self.heap[0..l], 0);
            Some(&self.heap[self.len])
        } else {
            None
        }
    }

    /// Return the number of elements in the heap
    pub fn len(&self) -> usize {
        self.len
    }

    /// Return true if the heap is empty
    pub fn is_empty(&self) -> bool {
        return self.len()==0
    }

    /// Return the number of elements the heap can store
    pub fn capacity(&self) -> usize {
        self.heap.len()
    }

    /// Return the smallest element or None if the heap is empty
    pub fn peek(&self) -> Option<&T> {
        if self.len>0 {
            Some(&self.heap[0])
        } else {
            None
        }
    }
}

/// Binary heap with bound
pub struct BoundedBinaryHeap<T>
{
    heap: Vec<T>,
    capacity: usize,
    len: usize
}

impl<T: PartialOrd+Clone> BoundedBinaryHeap<T>
{
    /// Constructor, create a new BoundedBinaryHeap with capacity
    /// ```
    /// extern crate rust_heap;
    /// use rust_heap::heap::BoundedBinaryHeap;
    /// let h:BoundedBinaryHeap<i32> = BoundedBinaryHeap::new(3);
    /// assert_eq!(h.capacity(), 3);
    /// assert_eq!(h.len(), 0);
    /// assert!(h.is_empty());
    /// ```
    pub fn new(capacity: usize) -> BoundedBinaryHeap<T> {
        BoundedBinaryHeap {
            heap: Vec::with_capacity(capacity),
            capacity: capacity,
            len: 0
        }
    }

    /// Constructor, create a new BoundedBinaryHeap from the content of a slice
    /// ```
    /// extern crate rust_heap;
    /// use rust_heap::heap::BoundedBinaryHeap;
    /// let h = BoundedBinaryHeap::from(&[1,2,3,4,5]);
    /// assert_eq!(h.capacity(), 5);
    /// assert_eq!(h.len(), 5);
    /// ```
    pub fn from(slice: &[T]) -> BoundedBinaryHeap<T> {
        let mut h = BoundedBinaryHeap {
            heap: slice.to_vec(),
            capacity: slice.len(),
            len: slice.len()
        };
        heapify(&mut h.heap);
        h
    }

    /// Constructor, create a new BoundedBinaryHeap from the content of a slice with given capacity
    /// ```
    /// extern crate rust_heap;
    /// use rust_heap::heap::BoundedBinaryHeap;
    /// let h = BoundedBinaryHeap::from_slice_with_capacity(&[1,2,3], 5);
    /// assert_eq!(h.capacity(), 5);
    /// assert_eq!(h.len(), 3);
    /// ```
    pub fn from_slice_with_capacity(slice: &[T], capacity: usize) -> BoundedBinaryHeap<T> {
        let mut h = BoundedBinaryHeap {
            heap: slice.to_vec(),
            capacity: capacity,
            len: slice.len()
        };
        heapify(&mut h.heap);
        h
    }

    /// Push a value into heap if value>=root and keep heap structure
    /// Overflow and return the smallest element when heap is full
    /// ```
    /// extern crate rust_heap;
    /// use rust_heap::heap::BoundedBinaryHeap;
    /// let mut h = BoundedBinaryHeap::from(&[1,2,3]);
    /// assert_eq!(h.push(4).unwrap(), 1);
    /// assert_eq!(h.len(), 3);
    /// ```
    pub fn push(&mut self, mut elem: T) -> Option<T> {
        if self.len<self.capacity() {
            self.len+=1;
            self.heap.push(elem);
            let l=self.len;
            sift_up(&mut self.heap[0..l]);
            None
        } else if (self.len>0) && (elem>=self.heap[0]) {
            mem::swap(&mut elem, &mut self.heap[0]);
            sift_down(&mut self.heap, 0);
            Some(elem)
        } else {
            Some(elem)
        }
    }

    /// Pop the smallest value from heap
    /// ```
    /// extern crate rust_heap;
    /// use rust_heap::heap::BoundedBinaryHeap;
    /// let mut h = BoundedBinaryHeap::from(&[3,2,1]);
    /// assert_eq!(h.pop().unwrap(), 1);
    /// ```
    pub fn pop(&mut self) -> Option<T> {
        if self.len>0 {
            self.heap.swap(0, self.len-1);
            self.len-=1;
            let l=self.len;
            sift_down(&mut self.heap[0..l], 0);
            Some(self.heap.swap_remove(self.len))
        } else {
            None
        }
    }

    /// Return the number of elements in the heap
    pub fn len(&self) -> usize {
        self.len       
    }

    /// Return true if the heap is empty
    pub fn is_empty(&self) -> bool {
        return self.len()==0
    }

    /// Return the number of elements the heap can store
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    /// Return the smallest element or None if the heap is empty
    pub fn peek(&self) -> Option<&T> {
        if self.len>0 {
            Some(&self.heap[0])
        } else {
            None
        }
    }
}
