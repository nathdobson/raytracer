use crate::tree::bvh::{BvhForest, BvhTree};

impl BvhForest {
    pub fn hybrid(mut self) -> BvhTree {
        self.contract_once();
        self.subdivide()
    }
}