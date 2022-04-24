use crate::math::scalar::Scalar;
use crate::geo::ray::Ray;
use crate::math::vec::{Vec2, Vec3};


pub struct View {
    fov: Vec2<f64>,
    orig: Vec3<f64>,
}

impl View {
    pub fn from_fov(orig: Vec3<f64>, fov: Vec2<f64>) -> Self { View { fov, orig } }
    pub fn get_ray<T: Scalar>(&self, viewpoint: Vec2<T>) -> Ray<T> {
        let x = viewpoint.x() * T::from((self.fov.x() / 2.0).tan());
        let y = viewpoint.y() * T::from((self.fov.y() / 2.0).tan());
        let z = T::from(-1.0);
        let d = Vec3::from([x, y, z]).normalize();
        Ray::new(self.orig.cast(), d)
    }
}