use crate::geo::ray::Ray;
use crate::math::scalar::Scalar;
use crate::render::object::{Manifold, Object, RaycastPoint};
use crate::geo::transform::Transform;

pub struct TransformObject<O> {
    transform: Transform<f64>,
    inner: O,
}

impl<O> TransformObject<O> {
    pub fn new(transform: Transform<f64>, inner: O) -> Self {
        TransformObject { transform, inner }
    }
}

impl<O: Object> Object for TransformObject<O> {
    fn raycast<T: Scalar>(&self, ray_outer: &Ray<T>, manifold: Option<Manifold>) -> Option<RaycastPoint<T>> {
        let transform = self.transform.cast::<T>();
        let ray_inner = transform.reverse_ray(ray_outer);
        let point = self.inner.raycast(&ray_inner, manifold)?;
        Some(RaycastPoint {
            position: ray_outer.pos(point.time),
            inter_normal: transform.forward_norm(point.inter_normal),
            geo_normal: transform.forward_norm(point.geo_normal),
            ..point
        })
    }
}