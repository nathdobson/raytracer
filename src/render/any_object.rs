use crate::math::scalar::Scalar;
use crate::geo::ray::Ray;
use crate::render::mesh_object::MeshObject;
use crate::render::object::{Manifold, Object, RaycastPoint};
use crate::geo::sphere::Sphere;
use crate::Plane;
use crate::render::plane_object::PlaneObject;
use crate::render::sphere_object::SphereObject;

pub enum AnyObject {
    Sphere(SphereObject),
    Model(MeshObject),
    Plane(PlaneObject),
}

impl Object for AnyObject {
    fn raycast<T: Scalar>(&self, ray: &Ray<T>, manifold: Option<Manifold>) -> Option<RaycastPoint<T>> {
        match self {
            AnyObject::Sphere(x) => x.raycast(ray, manifold),
            AnyObject::Model(x) => x.raycast(ray, manifold),
            AnyObject::Plane(x) => x.raycast(ray, manifold),
        }
    }
}