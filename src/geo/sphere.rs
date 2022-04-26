use std::f64::consts::PI;
use std::ops::Mul;
use rand::Rng;
use crate::geo::ray::Ray;
use roots::find_roots_quadratic;
use crate::geo::color::Color;
use crate::math::mat::Mat2;
use crate::math::scalar::{Der, Scalar};
use crate::render::object::{Manifold, RaycastPoint};
use crate::math::vec::{Vec2, Vec3};
use crate::render::material::Material;

#[derive(Debug)]
pub struct Sphere {
    orig: Vec3<f64>,
    rad: f64,
}

#[derive(Copy, Clone, Debug)]
pub struct ZenithY<T>(pub Vec2<T>);

impl<T: Scalar> ZenithY<T> {
    pub fn zenith(&self) -> T {
        self.0.x()
    }
    pub fn y(&self) -> T {
        self.0.y()
    }
    pub fn into_normal(&self) -> Vec3<T> {
        let radius = (T::from(1.0) - self.y() * self.y()).sqrt();
        let x = self.zenith().cos() * radius;
        let z = self.zenith().sin() * radius;
        Vec3::new(x, self.y(), z)
    }
}

impl ZenithY<f64> {
    pub fn as_input(&self) -> ZenithY<Der<2>> {
        ZenithY(self.0.as_input())
    }
}

pub struct Fibonacci<T>(Vec2<T>);

#[derive(Copy, Clone)]
pub struct FibonacciProjection {
    phi: f64,
    dydi: f64,
    dydj: f64,
}

impl FibonacciProjection {
    pub fn new(count: usize) -> Self {
        let count = count as f64;
        let phi = PI * (3.0 - (5.0f64).sqrt());
        let d = 2.0 * PI / phi;
        let dydi = 2.0 / (count + d);
        let dydj = dydi * d;
        FibonacciProjection { phi, dydi, dydj }
    }
    fn apply(self, rhs: Vec2<f64>) -> ZenithY<f64> {
        ZenithY(Vec2::new((self.phi * rhs.x()) % (2.0 * PI), -1.0 + self.dydi * rhs.x() + self.dydj * rhs.y()))
    }
}

impl Sphere {
    pub fn new(orig: Vec3<f64>, rad: f64) -> Self {
        Sphere { orig, rad }
    }
    pub fn orig(&self) -> Vec3<f64> { self.orig }
    pub fn rad(&self) -> f64 { self.rad }
    pub fn fibonacci_sphere(count: usize, rng: &mut impl Rng) -> Vec<ZenithY<f64>> {
        let proj = FibonacciProjection::new(count);
        (0..count).map(|i| {
            let i = i as f64 + rng.gen_range(0.0..1.0);
            let j = rng.gen_range(0.0..1.0);
            let v = Vec2::new(i, j);
            proj.apply(v)
        }).collect()
    }
    pub fn raycast<T: Scalar>(&self, ray: &Ray<T>) -> Option<RaycastPoint<T>> {
        let e = ray.orig();
        let d = ray.dir();
        let o: Vec3<T> = self.orig().cast();
        let r = self.rad();
        let a = d.dot(d);
        let b = T::from(2.0) * (e - o).dot(d);
        let c = (e - o).dot(e - o) - T::from(self.rad * self.rad);
        let roots = find_roots_quadratic(a, b, c);
        roots
            .as_ref().iter()
            .filter(|x| **x >= T::from(0.0))
            .min_by(|x, y| T::real_cmp(**x, **y))
            .map(|t| {
                let position = ray.pos(*t);
                let disp = position - o;
                let norm = disp.normalize();
                RaycastPoint {
                    time: *t,
                    position,
                    inter_normal: norm,
                    geo_normal: norm,
                    manifold: Manifold::empty(),
                    manifold_point: Vec2::new(norm.x(), norm.y()),
                    material: Material::nan(),
                }
            })
    }
}

#[test]
fn test_projection() {
    let projection = FibonacciProjection::new(100);
    for i in 0..4 {
        for j in 0..=4 {
            for i2 in 0..=4 {
                let v2 = Vec2::new(i as f64 + (i2 as f64) / 4.0, j as f64 / 4.0);
                let p = (projection.apply(v2)).into_normal();
                //println!("{:?} {:?} {:?}", p.x(), p.y(), p.z());
            }
        }
    }
}