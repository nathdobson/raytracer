use crate::math::scalar::Scalar;
use crate::math::vec::Vec3;

#[derive(Debug)]
pub struct Ray<T> {
    orig: Vec3<T>,
    dir: Vec3<T>,
}

const EPSILON: f64 = 1e-5;

impl<T: Scalar> Ray<T> {
    pub fn new(orig: Vec3<T>, dir: Vec3<T>) -> Self {
        Ray { orig, dir }
    }
    pub fn new_bounce(orig: Vec3<T>, dir: Vec3<T>) -> Self {
        Self::new(orig + dir * T::from(EPSILON), dir)
    }
    pub fn orig(&self) -> Vec3<T> { self.orig }
    pub fn dir(&self) -> Vec3<T> { self.dir }
    pub fn pos(&self, time: T) -> Vec3<T> {
        self.orig + self.dir * time
    }
}
