use roots::find_roots_quadratic;
use crate::{Sphere, Vec3};
use crate::geo::color::Color;
use crate::math::scalar::Scalar;
use crate::geo::ray::Ray;
use crate::render::material::Material;
use crate::render::object::{Manifold, Object, RaycastPoint};

pub struct SphereObject {
    sphere: Sphere,
    material: Material,
}

impl SphereObject {
    pub fn new(sphere: Sphere, material: Material) -> Self {
        SphereObject { sphere, material }
    }
}

impl Object for SphereObject {
    fn raycast<T: Scalar>(&self, ray: &Ray<T>, manifold: Option<Manifold>) -> Option<RaycastPoint<T>> {
        let point = self.sphere.raycast(ray)?;
        Some(RaycastPoint { material: self.material, ..point })
    }
}
