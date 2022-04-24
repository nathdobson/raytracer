use std::borrow::{Borrow, BorrowMut};
use std::fmt::{Debug, Formatter};
use std::ops::{Deref, DerefMut, Range};

pub struct SeqTree<T>([T]);

#[derive(Clone)]
enum SeqTreeViewRange {
    Node {
        left: Range<usize>,
        center: usize,
        right: Range<usize>,
    },
    Empty,
}

#[derive(Copy, Clone)]
pub enum SeqTreeView<'a, T> {
    Node {
        left: &'a SeqTree<T>,
        center: &'a T,
        right: &'a SeqTree<T>,
    },
    Empty,
}

pub enum SeqTreeViewMut<'a, T> {
    Node {
        left: &'a mut SeqTree<T>,
        center: &'a mut T,
        right: &'a mut SeqTree<T>,
    },
    Empty,
}

impl SeqTreeViewRange {
    pub fn new(range: Range<usize>) -> Self {
        if range.is_empty() {
            SeqTreeViewRange::Empty
        } else {
            let center = (range.end - range.start) / 2;
            SeqTreeViewRange::Node {
                left: range.start..center,
                center,
                right: center + 1..range.end,
            }
        }
    }
}

impl<T> SeqTree<T> {
    pub fn new(slice: &[T]) -> &Self { unsafe { &*(slice as *const [T] as *const Self) } }
    pub fn new_mut(slice: &mut [T]) -> &mut Self { unsafe { &mut *(slice as *mut [T] as *mut Self) } }
    pub fn as_view(&self) -> SeqTreeView<T> {
        match SeqTreeViewRange::new(0..self.0.len()) {
            SeqTreeViewRange::Node { left, center, right } => SeqTreeView::Node {
                left: Self::new(&self.0[left]),
                center: &self.0[center],
                right: Self::new(&self.0[right]),
            },
            SeqTreeViewRange::Empty => SeqTreeView::Empty,
        }
    }
    pub fn as_view_mut(&mut self) -> SeqTreeViewMut<T> {
        match SeqTreeViewRange::new(0..self.0.len()) {
            SeqTreeViewRange::Node { left, center, right } => {
                let (left, nonleft) = self.0.split_at_mut(center);
                let (center, right) = nonleft.split_first_mut().unwrap();
                SeqTreeViewMut::Node { left: Self::new_mut(left), center, right: Self::new_mut(right) }
            }
            SeqTreeViewRange::Empty => SeqTreeViewMut::Empty,
        }
    }
    pub fn build<O, F>(&mut self, key: F) where F: FnMut(&T) -> O, O: Ord {
        match SeqTreeViewRange::new(0..self.0.len()) {
            SeqTreeViewRange::Node { mut left, center, mut right } => {
                self.0.select_nth_unstable_by_key(center, key);
            }
            SeqTreeViewRange::Empty => {}
        }
    }
}

impl<T> Deref for SeqTree<T> {
    type Target = [T];
    fn deref(&self) -> &Self::Target { &self.0 }
}

impl<T> DerefMut for SeqTree<T> {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
}

impl<T> AsRef<[T]> for SeqTree<T> { fn as_ref(&self) -> &[T] { &self.0 } }

impl<T> AsMut<[T]> for SeqTree<T> { fn as_mut(&mut self) -> &mut [T] { &mut self.0 } }

impl<T> Borrow<[T]> for SeqTree<T> { fn borrow(&self) -> &[T] { &self.0 } }

impl<T> BorrowMut<[T]> for SeqTree<T> { fn borrow_mut(&mut self) -> &mut [T] { &mut self.0 } }

impl<T: Debug> Debug for SeqTree<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.as_view() {
            SeqTreeView::Node { left, center, right } => {
                let mut s = f.debug_struct("SeqNode");
                s.field("value", &center);
                if !left.is_empty() {
                    s.field("left", &left);
                }
                if !right.is_empty() {
                    s.field("right", &right);
                }
                s.finish()
            }
            SeqTreeView::Empty => f.debug_struct("SeqLeaf").finish(),
        }
    }
}