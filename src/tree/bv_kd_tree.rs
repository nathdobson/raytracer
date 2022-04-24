use std::fmt::Debug;
use itertools::Itertools;
use rand::{Rng, SeedableRng};
use rand_xorshift::XorShiftRng;
use crate::geo::axis_plane::AxisPlane;
use crate::geo::bounds::Bounds;
use crate::tree::kd_tree::{KdEntry, KdIter, KdNeighbor, KdQuery, KdTree};
use crate::Vec3;

#[derive(Debug, Clone)]
pub struct BvKdEntry<T> {
    bounds: Bounds<f64>,
    value: T,
}

pub struct BvKdTree<T> {
    inner: KdTree<BvKdEntry<T>>,
}

#[derive(Debug)]
pub struct BvKdQuery {
    bounds: Bounds<f64>,
}

impl<T> BvKdEntry<T> {
    pub fn new(bounds: Bounds<f64>, value: T) -> Self { BvKdEntry { bounds, value } }
    pub fn value(&self) -> &T { &self.value }
}

impl<T> KdQuery<BvKdEntry<T>> for BvKdQuery {
    fn center(&self) -> Vec3<f64> {
        self.bounds.center()
    }
    fn distance(&self, node: &KdEntry<BvKdEntry<T>>) -> f64 {
        self.bounds.union(&node.value().bounds).surface_area()
    }
    fn min_distance(&self, space: AxisPlane) -> f64 {
        let mut new_bounds = self.bounds;
        if space.coordinate < new_bounds.min()[space.axis] {
            new_bounds.min_mut()[space.axis] = space.coordinate;
        }
        if space.coordinate > new_bounds.max()[space.axis] {
            new_bounds.max_mut()[space.axis] = space.coordinate;
        }
        new_bounds.surface_area()
    }

    fn min_distance_bounds(&self, bounds: &Bounds<f64>) -> f64 {
        todo!()
    }
}

impl<T> BvKdTree<T> {
    pub fn new(volumes: Vec<BvKdEntry<T>>) -> Self {
        let inner = KdTree::new(
            volumes.into_iter()
                .map(|x| KdEntry::new(x.bounds.center(), x))
                .collect());
        BvKdTree { inner }
    }
    pub fn nearest(&self, bounds: &Bounds<f64>, k: usize) -> Vec<KdNeighbor<BvKdEntry<T>>> where T: Debug {
        self.inner.nearest(&BvKdQuery { bounds: *bounds }, k)
    }
    pub fn nearest_iter(&self, bounds: Bounds<f64>) -> KdIter<BvKdEntry<T>, BvKdQuery> where T: Debug {
        self.inner.nearest_iter(BvKdQuery { bounds: bounds })
    }
}

#[cfg(test)]
mod test {
    use itertools::Itertools;
    use ordered_float::NotNan;
    use rand::{Rng, SeedableRng};
    use rand_xorshift::XorShiftRng;
    use crate::geo::bounds::Bounds;
    use crate::Vec3;
    use crate::tree::bv_kd_tree::{BvKdEntry, BvKdTree};
    use crate::tree::kd_tree::KdNeighbor;

    #[test]
    fn test() {
        let tree = BvKdTree::new(
            vec![
                BvKdEntry {
                    bounds: Bounds::new(
                        Vec3::new(0.0, 0.0, 0.0),
                        Vec3::new(1.0, 1.0, 1.0)),
                    value: 123,
                }]);
        let (nearest, ) = tree.nearest(&Bounds::new(
            Vec3::new(1.0, 1.0, 1.0),
            Vec3::new(2.0, 2.0, 2.0)), 1).into_iter().collect_tuple().unwrap();
        assert_eq!(nearest.distance, 24.0);
        assert_eq!(*nearest.entry.value().value(), 123);
    }

    #[test]
    #[ignore]
    fn test_random() {
        for seed in 1..=20 {
            let mut rng = XorShiftRng::seed_from_u64(seed);
            fn random_bounds(rng: &mut XorShiftRng) -> Bounds<f64> {
                let res = 1000.0f64;
                let dres = 10.0f64;
                let start = Vec3::new(
                    (rng.gen_range(0.0..res)).round(),
                    (rng.gen_range(0.0..res)).round(),
                    (rng.gen_range(0.0..res)).round());
                let delta = Vec3::new(
                    (rng.gen_range(0.0..dres)).round(),
                    (rng.gen_range(0.0..dres)).round(),
                    (rng.gen_range(0.0..dres)).round());
                Bounds::new(start, start + delta)
            }
            let mut entries = vec![];
            for i in 0..100 {
                entries.push(BvKdEntry::new(random_bounds(&mut rng), i));
            }
            let tree = BvKdTree::new(entries.clone());
            for i in 1..10 {
                let query = random_bounds(&mut rng);
                let found: Vec<_> = tree.nearest(&query, i).into_iter().map(|x| {
                    (NotNan::new(x.distance).unwrap(), *x.entry.value().value())
                }).collect();
                let mut actual: Vec<_> = entries.iter()
                    .map(|x| (NotNan::new(x.bounds.union(&query).surface_area()).unwrap(), x.value))
                    .collect();
                actual.sort();
                assert_eq!(&found, &actual[0..i.min(entries.len())]);
            }
        }
    }
}