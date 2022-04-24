use std::collections::BTreeSet;
use std::mem;
use arrayvec::ArrayVec;
use itertools::Itertools;

#[derive(Debug)]
pub struct BinaryHeap<T>(Vec<T>);

fn parent(index: usize) -> Option<usize> {
    if index == 0 { None } else { Some((index - 1) / 2) }
}

fn sift_up<T>(slice: &mut [T], mut child: usize) where T: Ord {
    loop {
        if let Some(parent) = parent(child) {
            if slice[parent] < slice[child] {
                slice.swap(parent, child);
                child = parent;
                continue;
            } else {
                break;
            }
        } else {
            break;
        }
    }
}

fn sift_down<T>(slice: &mut [T]) where T: Ord {
    let mut parent = 0;
    loop {
        let children = children(slice, parent);
        if let Some(max_child) = children.into_iter().max_by_key(|i| &slice[*i]) {
            if slice[parent] < slice[max_child] {
                slice.swap(parent, max_child);
                parent = max_child;
                continue;
            } else {
                break;
            }
        } else {
            break;
        }
    }
}

fn left<T>(slice: &[T], index: usize) -> Option<usize> {
    let left = index * 2 + 1;
    (left < slice.len()).then_some(left)
}

fn right<T>(slice: &[T], index: usize) -> Option<usize> {
    let right = index * 2 + 2;
    (right < slice.len()).then_some(right)
}

fn children<T>(slice: &[T], index: usize) -> ArrayVec<usize, 2> {
    [left(slice, index), right(slice, index)].into_iter().flatten().collect()
}


impl<T> BinaryHeap<T> {
    pub fn new() -> Self { BinaryHeap(Vec::new()) }
    pub fn with_capacity(cap: usize) -> Self { BinaryHeap(Vec::with_capacity(cap)) }
    pub fn push(&mut self, value: T) where T: Ord {
        let index = self.0.len();
        self.0.push(value);
        sift_up(&mut self.0, index);
    }
    pub fn pop(&mut self) -> Option<T> where T: Ord {
        if self.is_empty() {
            return None;
        }
        let result = self.0.swap_remove(0);
        sift_down(&mut self.0);
        Some(result)
    }
    pub fn peek(&self) -> Option<&T> {
        self.0.first()
    }
    pub fn push_pop(&mut self, new: T) -> T where T: Ord {
        if let Some(first) = self.0.first_mut() {
            if new > *first {
                return new;
            } else {
                let result = mem::replace(first, new);
                sift_down(&mut self.0);
                return result;
            }
        } else {
            return new;
        }
    }
    pub fn into_sorted_vec(mut self) -> Vec<T> where T: Ord {
        for i in (0..self.0.len()).rev() {
            self.0.swap(0, i);
            sift_down(&mut self.0[..i]);
        }
        self.0
    }
    pub fn len(&self) -> usize {
        self.0.len()
    }
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn is_heap(&self) -> bool where T: Ord {
        for (i, x) in self.0.iter().enumerate() {
            if let Some(parent) = parent(i) {
                if self.0[parent] < *x {
                    return false;
                }
            }
        }
        true
    }
}

#[cfg(test)]
mod test {
    use std::collections::BTreeSet;
    use itertools::Itertools;
    use crate::util::binary_heap::BinaryHeap;

    #[test]
    fn test_simple() {
        let mut heap = BinaryHeap::new();
        heap.push(2);
        assert!(heap.is_heap());
        heap.push(1);
        assert!(heap.is_heap());
        heap.push(3);
        assert!(heap.is_heap());
        assert_eq!(heap.peek(), Some(&3));
        assert_eq!(heap.pop(), Some(3));
        assert!(heap.is_heap());
        assert_eq!(heap.peek(), Some(&2));
        assert_eq!(heap.pop(), Some(2));
        assert!(heap.is_heap());
        assert_eq!(heap.peek(), Some(&1));
        assert_eq!(heap.pop(), Some(1));
        assert!(heap.is_heap());
        assert_eq!(heap.peek(), None);
        assert_eq!(heap.pop(), None);
    }

    #[test]
    fn test_exhaustive_push() {
        for len in 0..8 {
            for pushes in (0..len).permutations(len) {
                let mut heap = BinaryHeap::new();
                for push in pushes {
                    heap.push(push);
                    assert!(heap.is_heap());
                }
                assert!(heap.is_heap());
                for i in (0..len).rev() {
                    assert_eq!(heap.pop(), Some(i));
                }
                assert_eq!(heap.pop(), None);
            }
        }
    }

    struct Tester {
        test: BinaryHeap<usize>,
        control: std::collections::BinaryHeap<usize>,
    }

    #[derive(Copy, Clone, Debug)]
    enum Op {
        Push(usize),
        Pop,
        PushPop(usize),
    }

    impl Tester {
        pub fn new() -> Self {
            Tester { test: BinaryHeap::new(), control: std::collections::BinaryHeap::new() }
        }
        pub fn run_ops(&mut self, ops: &[Op]) {
            for op in ops {
                self.run_op(*op);
            }
        }
        pub fn run_op(&mut self, op: Op) {
            match op {
                Op::Push(x) => {
                    self.test.push(x);
                    self.control.push(x);
                }
                Op::Pop => {
                    let actual = self.test.pop();
                    let expected = self.control.pop();
                    assert_eq!(actual, expected);
                }
                Op::PushPop(x) => {
                    let actual = self.test.push_pop(x);
                    self.control.push(x);
                    let expected = self.control.pop().unwrap();
                    assert_eq!(actual, expected);
                }
            }
        }
    }

    #[test]
    fn test_exhaustive_push_and_pop() {
        for len in 0..4 {
            let ops: Vec<_> = (0..len).map(Op::Push).chain((0..len).map(|_| Op::Pop)).collect();
            for ops in ops.iter().cloned().permutations(len) {
                let mut tester = Tester::new();
                tester.run_ops(ops.as_slice());
            }
        }
    }

    #[test]
    fn test_exhaustive_push_and_pop_and_push_pop() {
        for len in 0..5 {
            let ops: Vec<_> = (0..len).map(Op::Push)
                .chain((0..len).map(Op::PushPop))
                .chain((0..len).map(|_| Op::Pop)).collect();
            for ops in ops.iter().cloned().permutations(len) {
                let mut tester = Tester::new();
                tester.run_ops(ops.as_slice());
            }
        }
    }
}