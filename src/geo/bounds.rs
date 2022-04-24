use std::mem::swap;
use std::ops::Range;
use arrayvec::ArrayVec;
use itertools::Itertools;
use rand::prelude::Distribution;
use rand::Rng;
use crate::math::scalar::Scalar;
use crate::geo::ray::Ray;
use crate::math::vec::Vec3;
use crate::util::itertools2::Itertools2;

#[derive(Copy, Clone, Debug)]
pub struct Bounds<T> {
    min: Vec3<T>,
    max: Vec3<T>,
}

pub struct Interval<T> {
    min: T,
    max: T,
}

impl<T: Scalar> Bounds<T> {
    pub fn new(min: Vec3<T>, max: Vec3<T>) -> Self { Bounds { min, max } }
    pub fn empty() -> Self {
        Bounds { min: Vec3::broadcast(f64::INFINITY.into()), max: Vec3::broadcast(f64::NEG_INFINITY.into()) }
    }
    pub fn union(&self, other: &Self) -> Bounds<T> {
        Bounds {
            min: self.min.zip(other.min).map(|(x, y)| T::minimum(x, y)),
            max: self.max.zip(other.max).map(|(x, y)| T::maximum(x, y)),
        }
    }
    pub fn min(&self) -> Vec3<T> { self.min }
    pub fn max(&self) -> Vec3<T> { self.max }
    pub fn min_mut(&mut self) -> &mut Vec3<T> { &mut self.min }
    pub fn max_mut(&mut self) -> &mut Vec3<T> { &mut self.max }
    pub fn center(&self) -> Vec3<T> {
        (self.min + self.max) / T::from(2.0)
    }
    pub fn distance(&self, p: Vec3<T>) -> T {
        let mut total = T::from(0.0);
        for i in 0..3 {
            let d;
            if p[i] < self.min[i] {
                d = self.min[i] - p[i];
            } else if p[i] > self.max[i] {
                d = p[i] - self.max[i];
            } else {
                d = T::from(0.0);
            }
            total += d * d;
        }
        total.sqrt()
    }
    // pub fn corners(&self) -> [Vec3<T>; 8] {
    //     [
    //         Vec3::new(self.min.x(), self.min.y(), self.min.z()),
    //         Vec3::new(self.min.x(), self.min.y(), self.max.z()),
    //         Vec3::new(self.min.x(), self.max.y(), self.min.z()),
    //         Vec3::new(self.min.x(), self.max.y(), self.max.z()),
    //         Vec3::new(self.max.x(), self.min.y(), self.min.z()),
    //         Vec3::new(self.max.x(), self.min.y(), self.max.z()),
    //         Vec3::new(self.max.x(), self.max.y(), self.min.z()),
    //         Vec3::new(self.max.x(), self.max.y(), self.max.z()),
    //     ]
    // }

    pub fn range(&self, index: usize) -> Range<T> { self.min[index]..self.max[index] }
    pub fn dim(&self, index: usize) -> T {
        self.max[index] - self.min[index]
    }
    pub fn surface_area(&self) -> T {
        let (dx, dy, dz) = (self.dim(0), self.dim(1), self.dim(2));
        (dx * dy + dx * dz + dy * dz) * T::from(2.0)
    }
    pub fn cast<T2: From<T>>(&self) -> Bounds<T2> {
        Bounds { min: self.min.cast(), max: self.max.cast() }
    }
    pub fn raycast(&self, ray: &Ray<T>) -> Option<Interval<T>> {
        let mut interval = Interval::full();
        for i in 0..3 {
            let mut a = (self.min[i] - ray.orig()[i]) / ray.dir()[i];
            let mut b = (self.max[i] - ray.orig()[i]) / ray.dir()[i];
            if a.is_finite() && b.is_finite() {
                interval = interval.intersect(&Interval::new(a.minimum(b), a.maximum(b)))?;
            }
        }
        Some(interval)
    }
}

impl<T: Scalar> Interval<T> {
    pub fn new(min: T, max: T) -> Self {
        Interval { min, max }
    }
    pub fn full() -> Self {
        Interval { min: f64::NEG_INFINITY.into(), max: f64::INFINITY.into() }
    }
    pub fn intersect(&self, other: &Self) -> Option<Self> {
        let min = self.min.maximum(other.min);
        let max = self.max.minimum(other.max);
        if min < max {
            Some(Interval { min, max })
        } else {
            None
        }
    }
}

impl<T: Copy> From<Vec3<T>> for Bounds<T> {
    fn from(x: Vec3<T>) -> Self { Bounds { min: x, max: x } }
}

impl<T: Scalar> FromIterator<Bounds<T>> for Bounds<T> {
    fn from_iter<I: IntoIterator<Item=Bounds<T>>>(iter: I) -> Self {
        iter.into_iter().fold(Bounds::empty(), |x, y| x.union(&y))
    }
}

impl Distribution<Vec3<f64>> for Bounds<f64> {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Vec3<f64> {
        Vec3::new(
            rng.gen_range(self.range(0)),
            rng.gen_range(self.range(1)),
            rng.gen_range(self.range(2)))
    }
}