use crate::geo::ray::Ray;
use crate::math::scalar::Scalar;
use crate::render::object::{Manifold, Object, RaycastPoint};
use crate::{Vec2, Vec3};
use crate::geo::color::Color;
use crate::render::material::Material;

pub struct PlaneObject {
    position: Vec3<f64>,
    tan1: Vec3<f64>,
    tan2: Vec3<f64>,
    material1: Material,
    material2: Material,
}

impl PlaneObject {
    pub fn new(position: Vec3<f64>, tan1: Vec3<f64>, tan2: Vec3<f64>, material1: Material, material2: Material) -> Self {
        PlaneObject { position, tan1, tan2, material1, material2 }
    }
}

impl Object for PlaneObject {
    fn raycast<T: Scalar>(&self, ray: &Ray<T>, manifold: Option<Manifold>) -> Option<RaycastPoint<T>> {
        let p = self.position.cast();
        let n = self.tan1.cross(self.tan2).cast();
        let e = ray.orig();
        let d = ray.dir();
        let t = (p - e).dot(n) / d.dot(n);
        if t < T::from(0.0) {
            return None;
        }
        let x = ray.pos(t);
        let m1 = (x - p).dot(self.tan1.cast());
        let m2 = (x - p).dot(self.tan2.cast());
        let scale = 10.0;
        let mut material = self.material1;
        if ((m1.into_const() * scale).round() as i64 + (m2.into_const() * scale).round() as i64) % 2 == 0 {
            material = self.material2;
        }
        Some(RaycastPoint {
            time: t,
            position: x,
            inter_normal: n,
            geo_normal: n,
            material,
            manifold: Manifold::empty(),
            manifold_point: Vec2::new(m1, m2),
        })
    }
}