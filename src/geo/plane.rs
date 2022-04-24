use crate::geo::ray::Ray;
use crate::math::vec::Vec3;

pub struct Plane<T> {
    orig: Vec3<T>,
    norm: Vec3<T>,
}

impl<T> Plane<T> {
    pub fn new(orig: Vec3<T>, norm: Vec3<T>) -> Self {
        Plane { orig, norm }
    }
    pub fn raycast(&self, ray: &Ray<T>) -> Option<T> {
        todo!()
    }
}