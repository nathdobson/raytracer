use std::ops::Range;
use itertools::Itertools;
use partition::partition;
use ordered_float::NotNan;
use crate::geo::bounds::{Bounds, Interval};
use crate::math::scalar::Scalar;
use crate::geo::ray::Ray;
use crate::mesh::{Mesh, MeshFace, TriMesh, TriVerts};
use crate::render::object::{Manifold, RaycastPoint, RaycastPointHolder};
use partition::partition_index;
use crate::tree::kd_tree::KdTree;
use crate::util::itertools2::Itertools2;

struct BvhEntry {
    bounds: Bounds<f64>,
    child_nodes: Range<usize>,
    child_leaves: Range<usize>,
}

pub struct Bvh {
    nodes: Vec<BvhEntry>,
    leaves: Vec<TriVerts>,
    root: usize,
}

#[derive(Debug)]
pub struct BvhTree {
    leaves: Vec<TriVerts>,
    nodes: Vec<BvhTree>,
    bounds: Bounds<f64>,
}

#[derive(Debug)]
pub struct BvhForest {
    pub trees: Vec<BvhTree>,
}

impl BvhEntry {
    pub fn new(tris: &Vec<TriVerts>, child_leaves: Range<usize>) -> Self {
        BvhEntry {
            bounds: tris[child_leaves.clone()].iter().map(|x| x.triangle().bounds()).collect(),
            child_nodes: 0..0,
            child_leaves,
        }
    }
}

impl BvhForest {
    pub fn new(mesh: &[TriVerts]) -> Self {
        BvhForest {
            trees: mesh.into_iter().map(|face| {
                let mut tree = BvhTree::new();
                tree.add_leaf(&face);
                tree
            }).collect()
        }
    }
}

impl BvhTree {
    pub fn new() -> Self {
        BvhTree {
            leaves: vec![],
            nodes: vec![],
            bounds: Bounds::empty(),
        }
    }
    pub fn add_leaf(&mut self, leaf: &TriVerts) {
        self.bounds = self.bounds.union(&leaf.triangle().bounds());
        self.leaves.push(leaf.clone());
    }
    pub fn add_node(&mut self, node: BvhTree) {
        self.bounds = self.bounds.union(&node.bounds);
        self.nodes.push(node);
    }
    pub fn bounds(&self) -> &Bounds<f64> { &self.bounds }
}

impl Bvh {
    pub fn new(tree: &BvhTree) -> Bvh {
        let mut bvh = Bvh {
            nodes: vec![],
            leaves: vec![],
            root: 0,
        };
        let root = bvh.add_bvh_tree(tree);
        bvh.root = bvh.nodes.len();
        bvh.nodes.push(root);
        bvh
    }
    fn add_bvh_tree(&mut self, tree: &BvhTree) -> BvhEntry {
        let leaf_start = self.leaves.len();
        self.leaves.extend(tree.leaves.iter().cloned());
        let leaf_end = self.leaves.len();
        let nodes: Vec<_> = tree.nodes.iter().map(|child| {
            self.add_bvh_tree(child)
        }).collect();
        let nodes_start = self.nodes.len();
        self.nodes.extend(nodes);
        let nodes_end = self.nodes.len();
        BvhEntry {
            bounds: tree.bounds,
            child_nodes: nodes_start..nodes_end,
            child_leaves: leaf_start..leaf_end,
        }
    }
    pub fn raycast<T: Scalar>(&self, ray: &Ray<T>, manifold: Option<Manifold>) -> Option<RaycastPoint<T>> {
        if let Some(manifold) = manifold {
            let (inner, index) = manifold.pop().unwrap();
            let point = self.leaves[index].raycast(ray, true)?;
            Some(RaycastPoint { manifold: point.manifold.push(index), ..point })
        } else {
            let mut holder = RaycastPointHolder::new();
            self.raycast_rec(ray, self.root, &mut holder);
            holder.into_point()
        }
    }
    pub fn raycast_rec<T: Scalar>(&self, ray: &Ray<T>, node: usize, output: &mut RaycastPointHolder<T>) {
        let node = &self.nodes[node];
        if let Some(bounds_times) = node.bounds.cast::<T>().raycast(ray) {
            if bounds_times.intersect(&output.interval()).is_some() {
                for child in node.child_leaves.clone() {
                    let f = &self.leaves[child];
                    output.add(f.raycast(ray, false).map(|point| {
                        RaycastPoint { manifold: point.manifold.push(child), ..point }
                    }))
                }
                for child in node.child_nodes.clone() {
                    self.raycast_rec(ray, child, output);
                }
            }
        }
    }
    pub fn bounds(&self) -> Bounds<f64> {
        self.nodes[self.root].bounds
    }
}