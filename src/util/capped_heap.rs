use crate::util::binary_heap::BinaryHeap;

pub struct CappedHeap<T> {
    heap: BinaryHeap<T>,
    cap: usize,
}

impl<T> CappedHeap<T> {
    pub fn with_capacity(cap: usize) -> Self {
        CappedHeap {
            heap: BinaryHeap::with_capacity(cap),
            cap,
        }
    }
    pub fn push(&mut self, x: T) where T: Ord {
        if self.heap.len() == self.cap {
            self.heap.push_pop(x);
        } else {
            self.heap.push(x);
        }
    }
    pub fn peek(&self) -> Option<&T> {
        self.heap.peek()
    }
    pub fn into_sorted_vec(self) -> Vec<T> where T: Ord {
        self.heap.into_sorted_vec()
    }
    pub fn len(&self) -> usize {
        self.heap.len()
    }
    pub fn capacity(&self) -> usize {
        self.cap
    }
}