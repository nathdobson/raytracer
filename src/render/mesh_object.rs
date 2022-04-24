use std::sync::Arc;
use crate::geo::ray::Ray;
//use crate::bvh::BVH;
use crate::geo::color::Color;
use crate::math::scalar::Scalar;
use crate::render::material::Material;
use crate::tree::bvh::Bvh;
use crate::render::object::{Manifold, Object, RaycastPoint};

pub struct MeshObject {
    mesh: Arc<Bvh>,
    material: Material,
}

impl MeshObject {
    pub fn new(mesh: Arc<Bvh>, material: Material) -> Self {
        MeshObject { mesh, material }
    }
}


impl Object for MeshObject {
    fn raycast<T: Scalar>(&self, ray: &Ray<T>, manifold: Option<Manifold>) -> Option<RaycastPoint<T>> {
        let point = self.mesh.raycast(ray, manifold)?;
        Some(RaycastPoint { material: self.material, ..point })
    }
}