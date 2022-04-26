use std::cmp::Ordering;
use std::mem;
use arrayvec::ArrayVec;
use crate::geo::ray::Ray;
use ordered_float::NotNan;
use crate::geo::bounds::Interval;
use crate::geo::color::Color;
use crate::math::scalar::Scalar;
use crate::math::vec::{Vec2, Vec3};
use crate::render::material::Material;

#[derive(Copy, Clone, Debug, Eq, Ord, PartialOrd, PartialEq)]
pub struct Manifold {
    array: [u32; 2],
    len: usize,
}

#[derive(Clone, Debug)]
pub struct RaycastPoint<T> {
    pub time: T,
    pub position: Vec3<T>,
    pub inter_normal: Vec3<T>,
    pub geo_normal: Vec3<T>,
    pub manifold: Manifold,
    pub manifold_point: Vec2<T>,
    pub material: Material,
}

pub struct RaycastPointHolder<T> {
    point: Option<RaycastPoint<T>>,
}

pub trait Object: Sync {
    fn raycast<T: Scalar>(&self, ray: &Ray<T>, manifold: Option<Manifold>) -> Option<RaycastPoint<T>>;
}

impl Manifold {
    pub fn empty() -> Self {
        Manifold { array: [0; 2], len: 0 }
    }
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
    pub fn push(mut self, other: usize) -> Self {
        assert!(self.len < self.array.len());
        self.array[self.len] = u32::try_from(other).unwrap();
        self.len += 1;
        self
    }
    pub fn pop(mut self) -> Option<(Self, usize)> {
        if self.len == 0 {
            return None;
        }
        self.len -= 1;
        let end = mem::replace(&mut self.array[self.len], 0);
        Some((self, end as usize))
    }
}

impl<T: Scalar> RaycastPointHolder<T> {
    pub fn new() -> Self { RaycastPointHolder { point: None } }
    pub fn add(&mut self, new: Option<RaycastPoint<T>>) {
        if let Some(new) = new {
            if let Some(old) = &mut self.point {
                if new.time < old.time {
                    self.point = Some(new)
                }
            } else {
                self.point = Some(new)
            }
        }
    }
    pub fn interval(&self) -> Interval<T> {
        Interval::new(T::from(0.0), self.point.as_ref().map_or(T::from(f64::INFINITY), |p| p.time))
    }
    pub fn into_point(self) -> Option<RaycastPoint<T>> {
        self.point
    }
}

impl<T: Scalar> PartialEq for RaycastPoint<T> {
    fn eq(&self, other: &Self) -> bool {
        self.time.real_eq(other.time)
    }
}

impl<T: Scalar> Eq for RaycastPoint<T> {}

impl<T: Scalar> PartialOrd for RaycastPoint<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.time.real_cmp(other.time))
    }
}

impl<T: Scalar> Ord for RaycastPoint<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.time.real_cmp(other.time)
    }
}