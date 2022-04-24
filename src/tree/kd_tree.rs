use std::cmp::{Ordering, Reverse};
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::fmt::{Debug, Formatter};
use std::ops::Range;
use ordered_float::NotNan;
use rand::{Rng, thread_rng};
use crate::util::binary_heap::BinaryHeap;
use crate::geo::bounds::Bounds;
use crate::util::capped_heap::CappedHeap;
use crate::math::vec::{Vec3, Vector};
use rand_xorshift::XorShiftRng;
use rand::SeedableRng;
use crate::geo::axis_plane::{AxisPlane};
use crate::math::scalar::Scalar;
use crate::math::scalar_key::ScalarKey;
use crate::tree::seq_tree::{SeqTree, SeqTreeView, SeqTreeViewMut};

#[derive(Copy, Clone, Debug)]
pub struct KdEntry<T> {
    axis: u8,
    bounds: Bounds<f64>,
    position: Vector<3, f64>,
    value: T,
}

pub struct KdTree<T> {
    entries: Vec<KdEntry<T>>,
}

#[derive(Debug, Copy, Clone)]
pub struct KdNeighbor<'a, T> {
    pub distance: f64,
    pub entry: &'a KdEntry<T>,
}

#[derive(Debug)]
enum KdIterNode<'a, T> {
    KdEntry(&'a KdEntry<T>),
    KdNode(&'a SeqTree<KdEntry<T>>),
}

#[derive(Debug)]
pub struct KdIter<'a, T, Q> {
    query: Q,
    heap: BinaryHeap<Reverse<ScalarKey<f64, KdIterNode<'a, T>>>>,
}

pub trait KdQuery<T>: Debug {
    fn center(&self) -> Vec3<f64>;
    fn distance(&self, node: &KdEntry<T>) -> f64;
    fn min_distance(&self, space: AxisPlane) -> f64;
    fn min_distance_bounds(&self, bounds: &Bounds<f64>) -> f64;
}

impl<T> KdQuery<T> for Vec3<f64> {
    fn center(&self) -> Vec3<f64> { *self }
    fn distance(&self, node: &KdEntry<T>) -> f64 { Vec3::distance(*self, node.position) }
    fn min_distance(&self, plane: AxisPlane) -> f64 { (self[plane.axis] - plane.coordinate).abs() }
    fn min_distance_bounds(&self, bounds: &Bounds<f64>) -> f64 { bounds.distance(*self) }
}

impl<T> KdEntry<T> {
    pub fn new(position: Vec3<f64>, value: T) -> Self { KdEntry { axis: 0, bounds: Bounds::new(Vec3::nan(), Vec3::nan()), position, value } }
    pub fn position(&self) -> Vec3<f64> { self.position }
    pub fn value(&self) -> &T { &self.value }
}

#[derive(Debug)]
pub struct Stats {
    explored: BTreeMap<usize, usize>,
}

impl<T> KdTree<T> {
    pub fn new(mut entries: Vec<KdEntry<T>>) -> Self {
        let mut result = KdTree { entries };
        Self::build_rec(result.as_slice_mut());
        result
    }
    pub fn build_rec(tree: &mut SeqTree<KdEntry<T>>) -> Bounds<f64> {
        let bounds: Bounds<f64> = tree.iter().map(|x| Bounds::from(x.position)).collect();
        let axis = (0..3u8).max_by_key(|i| NotNan::try_from(bounds.dim(*i as usize)).unwrap()).unwrap();
        tree.build(|x| NotNan::try_from(x.position[axis as usize]).unwrap());
        let mut bounds = Bounds::empty();
        match tree.as_view_mut() {
            SeqTreeViewMut::Node { mut left, center, mut right } => {
                center.axis = axis;
                bounds = bounds.union(&Self::build_rec(left));
                bounds = bounds.union(&Self::build_rec(right));
                bounds = bounds.union(&Bounds::new(center.position, center.position));
                center.bounds = bounds
            }
            SeqTreeViewMut::Empty => {}
        }
        bounds
    }
    pub fn as_slice(&self) -> &SeqTree<KdEntry<T>> { SeqTree::new(self.entries.as_slice()) }
    pub fn as_slice_mut(&mut self) -> &mut SeqTree<KdEntry<T>> { SeqTree::new_mut(self.entries.as_mut_slice()) }
    fn assert_kd_tree(&self) {
        Self::assert_kd_tree_rec(self.as_slice())
    }
    fn assert_kd_tree_rec(tree: &SeqTree<KdEntry<T>>) {
        match tree.as_view() {
            SeqTreeView::Node { left, center, right } => {
                let axis = center.axis as usize;
                for x in left.as_ref() {
                    assert!(x.position[axis] <= center.position[axis]);
                }
                for x in right.as_ref() {
                    assert!(center.position[axis] <= x.position[axis]);
                }
            }
            SeqTreeView::Empty => {}
        }
    }
    pub fn nearest<Q: KdQuery<T>>(&self, query: &Q, count: usize) -> Vec<KdNeighbor<T>> where T: Debug {
        let mut heap = CappedHeap::with_capacity(count);
        let mut stats = Stats { explored: BTreeMap::new() };
        Self::nearest_rec(self.as_slice(), query, &mut heap, &mut stats);
        // println!("{:?}", stats);
        heap.into_sorted_vec()
    }
    pub fn nearest_rec<'a, Q: KdQuery<T>>(tree: &'a SeqTree<KdEntry<T>>, query: &Q, heap: &mut CappedHeap<KdNeighbor<'a, T>>, stats: &mut Stats) where T: Debug {
        match tree.as_view() {
            SeqTreeView::Node { left, center, right } => {
                if heap.len() == heap.capacity() {
                    let max_so_far = heap.peek().map_or(f64::NEG_INFINITY, |max| max.distance);
                    let min_possible = query.min_distance_bounds(&center.bounds);
                    if min_possible > max_so_far {
                        return;
                    }
                }
                let axis = center.axis as usize;
                let (first, second) = if query.center()[axis] < center.position[axis] {
                    (left, right)
                } else {
                    (right, left)
                };
                *stats.explored.entry(heap.len()).or_default() += 1;
                KdTree::nearest_rec(first, query, heap, stats);
                let neighbor = KdNeighbor { distance: query.distance(center), entry: center };
                heap.push(neighbor);
                let mut more = heap.len() < heap.capacity();
                if !more {
                    let max_so_far = heap.peek().map_or(f64::NEG_INFINITY, |max| max.distance);
                    let min_possible = query.min_distance(AxisPlane { axis, coordinate: center.position[axis] });
                    more = min_possible < max_so_far;
                }
                if more {
                    KdTree::nearest_rec(second, query, heap, stats);
                }
            }
            SeqTreeView::Empty => {}
        }
    }

    pub fn nearest_iter<Q: KdQuery<T>>(&self, query: Q) -> KdIter<T, Q> {
        let mut heap = BinaryHeap::new();
        heap.push(Reverse(ScalarKey::new(0.0, KdIterNode::KdNode(self.as_slice()))));
        KdIter { query, heap }
    }
}

impl<'a, T, Q: KdQuery<T>> Iterator for KdIter<'a, T, Q> {
    type Item = KdNeighbor<'a, T>;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let kv = self.heap.pop()?.0;
            match kv.value() {
                KdIterNode::KdEntry(v) => return Some(KdNeighbor { distance: *kv.key(), entry: v }),
                KdIterNode::KdNode(tree) => {
                    match tree.as_view() {
                        SeqTreeView::Node { left, center, right } => {
                            let axis = center.axis as usize;
                            let (first, second) = if self.query.center()[axis] < center.position[axis] {
                                (left, right)
                            } else {
                                (right, left)
                            };
                            self.heap.push(Reverse(ScalarKey::new(0.0, KdIterNode::KdNode(first))));
                            self.heap.push(Reverse(ScalarKey::new(self.query.min_distance(AxisPlane { axis, coordinate: center.position[axis] }), KdIterNode::KdNode(second))));
                            self.heap.push(Reverse(ScalarKey::new(self.query.distance(center), KdIterNode::KdEntry(center))));
                        }
                        SeqTreeView::Empty => {}
                    }
                }
            }
        }
    }
}

impl<T> Default for KdTree<T> {
    fn default() -> Self { KdTree { entries: vec![] } }
}

impl<'a, T> Eq for KdNeighbor<'a, T> {}

impl<'a, T> PartialEq<Self> for KdNeighbor<'a, T> {
    fn eq(&self, other: &Self) -> bool {
        self.distance.real_eq(other.distance)
    }
}

impl<'a, T> PartialOrd<Self> for KdNeighbor<'a, T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.distance.real_cmp(other.distance))
    }
}

impl<'a, T> Ord for KdNeighbor<'a, T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.distance.real_cmp(other.distance)
    }
}

impl<T: Debug> Debug for KdTree<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.as_slice().fmt(f)
    }
}

#[test]
fn test_simple() {
    let entries = vec![
        KdEntry::new(Vec3::new(1.0, 0.0, 0.0), 1),
        KdEntry::new(Vec3::new(2.0, 0.0, 0.0), 2),
        KdEntry::new(Vec3::new(3.0, 0.0, 0.0), 3),
    ];
    let tree = KdTree::new(entries.clone());
    assert_eq!(Vec::<KdNeighbor<usize>>::new(), tree.nearest(&Vec3::new(0.0, 0.0, 0.0), 0));
    assert_eq!(vec![KdNeighbor { distance: 1.0, entry: &entries[0] }],
               tree.nearest(&Vec3::new(0.0, 0.0, 0.0), 1));
    assert_eq!(vec![KdNeighbor { distance: 1.0, entry: &entries[0] },
                    KdNeighbor { distance: 2.0, entry: &entries[1] }],
               tree.nearest(&Vec3::new(0.0, 0.0, 0.0), 2));
    assert_eq!(vec![KdNeighbor { distance: 1.0, entry: &entries[0] },
                    KdNeighbor { distance: 2.0, entry: &entries[1] },
                    KdNeighbor { distance: 3.0, entry: &entries[2] }],
               tree.nearest(&Vec3::new(0.0, 0.0, 0.0), 3));
}

#[test]
fn test_random() {
    for seed in 1..=20 {
        let mut rng = XorShiftRng::seed_from_u64(seed);
        fn random_point(rng: &mut XorShiftRng) -> Vec3<f64> {
            let res = 1000.0f64;
            Vec3::new(
                (rng.gen_range(0.0..1.0) * res).round(),
                (rng.gen_range(0.0..1.0) * res).round(),
                (rng.gen_range(0.0..1.0) * res).round())
        }
        let mut entries = vec![];
        for i in 0..100 {
            entries.push(KdEntry::new(random_point(&mut rng), i));
        }
        let tree = KdTree::new(entries.clone());
        tree.assert_kd_tree();
        for i in 1..10 {
            let query = random_point(&mut rng);
            let found: Vec<_> = tree.nearest(&query, i).into_iter().map(|x| ScalarKey::new(x.distance, x.entry.value)).collect();
            let iter_found: Vec<_> = tree.nearest_iter(query).map(|x| ScalarKey::new(x.distance, x.entry.value)).collect();
            let mut actual: Vec<_> = entries.iter().map(|x| ScalarKey::new(query.distance(x.position), x.value)).collect();
            actual.sort();
            assert_eq!(&found, &actual[0..i.min(entries.len())]);
            assert_eq!(&iter_found, &actual);
        }
    }
}