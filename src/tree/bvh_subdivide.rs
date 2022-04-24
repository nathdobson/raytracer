use std::default::default;
use std::mem;
use ordered_float::NotNan;
use crate::geo::bounds::Bounds;
use crate::tree::bvh::{BvhForest, BvhTree};
use crate::util::itertools2::Itertools2;

impl BvhForest {
    pub fn subdivide(mut self) -> BvhTree {
        if self.trees.len() == 0 {
            return BvhTree::new()
        } else if self.trees.len() == 1 {
            return self.trees.into_iter().next().unwrap()
        }
        let (left, right) = self.subdivide_once();
        let mut node = BvhTree::new();
        node.add_node(left.subdivide());
        node.add_node(right.subdivide());
        node
    }
    pub fn subdivide_once(mut self) -> (Self, Self) {
        let bounds: Bounds<f64> = self.trees.iter().map(|x| x.bounds()).cloned().collect();
        let axis = (0..3)
            .map(|i| (i, NotNan::try_from(bounds.dim(i)).unwrap()))
            .max_by_key(|(i, d)| *d)
            .unwrap().0;
        let split_coordinate = bounds.min()[axis] + bounds.dim(axis) / 2.0;
        self.trees.sort_by_key(|x| NotNan::new(x.bounds().center()[axis]).unwrap());
        let mut split = self.trees.binary_search_by_key(
            &NotNan::try_from(split_coordinate).unwrap(),
            |x| NotNan::try_from(x.bounds().center()[axis]).unwrap()).map_or_else(|x| x, |x| x);
        const CLAMP: f64 = 0.01;
        let min_split = (((self.trees.len() as f64) * CLAMP) as usize).max(1);
        let max_split = self.trees.len() - min_split;
        if split < min_split {
            split = min_split;
        }
        if split > max_split {
            split = max_split;
        }
        let right = self.trees.split_off(split);
        (self, BvhForest { trees: right })
    }
}