use std::iter;
use itertools::Either;
use crate::render::any_object::AnyObject;
use crate::geo::ray::Ray;
use crate::math::scalar::Scalar;
use crate::render::object::{Manifold, Object, RaycastPoint};
use crate::render::transform_object::TransformObject;

pub struct SceneObject {
    objects: Vec<TransformObject<AnyObject>>,
}

impl SceneObject {
    pub fn new(objects: Vec<TransformObject<AnyObject>>) -> Self {
        SceneObject { objects }
    }
}

impl Object for SceneObject {
    fn raycast<T: Scalar>(&self, ray: &Ray<T>, manifold: Option<Manifold>) -> Option<RaycastPoint<T>> {
        let mut inner = None;
        let it;
        if let Some(manifold) = manifold {
            let r = manifold.pop().unwrap();
            inner = Some(r.0);
            let index = r.1;
            it = Either::Left(iter::once((index, &self.objects[index])));
        } else {
            it = Either::Right(self.objects.iter().enumerate());
        }
        it.flat_map(|(index, shape)| {
            let point = shape.raycast(&ray, inner)?;
            Some(RaycastPoint { manifold: point.manifold.push(index), ..point })
        }).min()
    }
}

