use crate::geo::bounds::Bounds;
use crate::geo::ray::Ray;
use ordered_float::NotNan;
use crate::math::scalar::Scalar;
use crate::math::vec::{Vec2, Vec3};

pub struct Triangle<T> {
    vertices: [Vec3<T>; 3],
}

pub struct TrianglePoint<T> {
    time: T,
    barycenter: Vec3<T>,
    position: Vec3<T>,
    manifold_point: Vec2<T>,
    geo_normal: Vec3<T>,
}

impl<T: Copy> TrianglePoint<T> {
    pub fn new(time: T, barycenter: Vec3<T>, position: Vec3<T>, manifold_point: Vec2<T>, geo_normal: Vec3<T>) -> Self {
        TrianglePoint {
            time,
            barycenter,
            position,
            manifold_point,
            geo_normal,
        }
    }
    pub fn time(&self) -> T { self.time }
    pub fn barycenter(&self) -> Vec3<T> { self.barycenter }
    pub fn position(&self) -> Vec3<T> { self.position }
    pub fn manifold_point(&self) -> Vec2<T> { self.manifold_point }
    pub fn geo_normal(&self) -> Vec3<T> { self.geo_normal }
}

impl<T: Scalar> Triangle<T> {
    pub fn new(vertices: [Vec3<T>; 3]) -> Self {
        Triangle { vertices }
    }
    pub fn vertices(&self) -> &[Vec3<T>; 3] {
        &self.vertices
    }
    pub fn bounds(&self) -> Bounds<T> {
        self.vertices.map(Bounds::from).into_iter().collect()
    }
    pub fn normal(&self) -> Vec3<T> {
        let [v0, v1, v2] = self.vertices;
        (v1 - v0).cross(v2 - v1).normalize()
    }
    pub fn cast<T2: From<T>>(self) -> Triangle<T2> {
        Triangle { vertices: self.vertices.map(|x| x.cast()) }
    }

    pub fn raycast(&self, ray: &Ray<T>, manifold: bool) -> Option<TrianglePoint<T>> {
        let [v0, v1, v2] = self.vertices;
        let nt = (v1 - v0).cross(v2 - v1);
        let denom = nt.dot(nt);
        if nt.dot(ray.dir()) > 0.0.into() && !manifold {
            return None;
        }
        let e = ray.orig();
        let d = ray.dir();
        let t = (v0 - e).dot(nt) / d.dot(nt);
        if t < 0.0.into() && !manifold {
            return None;
        }
        let q = ray.pos(t);
        let mut barycenter = Vec3::default();
        for i in 0..3 {
            let (va, vb) = (self.vertices[i], self.vertices[(i + 1) % 3]);
            let c = (vb - va).cross(q - va).dot(nt) / denom;
            if c < 0.0.into() && !manifold {
                return None;
            }
            barycenter[(i + 2) % 3] = c;
        }
        let error = (v0 * barycenter[0]
            + v1 * barycenter[1]
            + v2 * barycenter[2] - q).length();
        let manifold_point = Vec2::new(barycenter.x(), barycenter.y());
        Some(TrianglePoint::new(t, barycenter, q, manifold_point, nt.normalize()))
    }
}
