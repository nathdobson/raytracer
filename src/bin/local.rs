#![feature(default_free_fn)]
#![allow(unused_imports)]
#![allow(unused_variables)]

extern crate core;

use std::default::default;
use std::f64::consts::PI;
use std::fs;
use std::path::Path;
use std::sync::Arc;
use raytracer::render::any_object::AnyObject;
//use crate::bvh::BVH;
use raytracer::math::mat::Mat4;
use raytracer::render::renderer::{Light, Renderer};
use raytracer::render::scene_object::SceneObject;
use raytracer::geo::sphere::Sphere;
use raytracer::geo::transform::Transform;
use raytracer::render::transform_object::TransformObject;
use raytracer::math::vec::{Vec2, Vec3};
use raytracer::geo::view::View;

// mod window;

use ::rayon::ThreadPoolBuilder;
use raytracer::tree::bvh::Bvh;
use raytracer::mesh::{cow, pinecone};
use raytracer::geo::color::Color;
use raytracer::geo::plane::Plane;
use raytracer::geo::transform::TransformBuilder;
use raytracer::math::vec::Vector;
use raytracer::mesh::{bunny, sphere};
use raytracer::render::material::Material;
use raytracer::render::mesh_object::MeshObject;
use raytracer::render::plane_object::PlaneObject;
use raytracer::render::renderer::Scene;
use raytracer::render::sphere_object::SphereObject;
use raytracer::SceneBuilder;

fn main() {
    ThreadPoolBuilder::new().num_threads(8).build_global().unwrap();
    for i in 0..100 {
        let builder = SceneBuilder { time: i };
        let mut renderer = Renderer::new(builder.scene());
        renderer.render();
        let dir = Path::new("output/local").join(format!("{}", i));
        fs::create_dir_all(&dir).unwrap();
        for (name, image) in renderer.images() {
            fs::write(dir.join(name), image).unwrap();
        }
    }
}