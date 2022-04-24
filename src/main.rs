#![feature(box_syntax)]
#![feature(trait_alias)]
#![feature(generic_const_exprs)]
#![feature(array_zip)]
#![feature(float_minimum_maximum)]
#![feature(default_free_fn)]
#![feature(bool_to_option)]
#![feature(map_first_last)]
#![allow(unused_variables, dead_code, unused_imports, incomplete_features, unused_assignments, unused_mut, unreachable_code)]
#![deny(unused_must_use)]
#![feature(type_alias_impl_trait)]
#![feature(test)]
#![feature(bench_black_box)]


extern crate core;

use std::sync::Arc;
use render::any_object::AnyObject;
//use crate::bvh::BVH;
use math::mat::Mat4;
use render::renderer::{Light, Renderer};
use render::scene_object::SceneObject;
use geo::sphere::Sphere;
use geo::transform::Transform;
use render::transform_object::TransformObject;
use math::vec::{Vec2, Vec3};
use geo::view::View;

mod util;
mod math;
mod render;
mod geo;
mod mesh;
mod tree;
// mod window;

use ::rayon::ThreadPoolBuilder;
use tree::bvh::Bvh;
use mesh::{cow, pinecone};
use crate::geo::color::Color;
use crate::geo::plane::Plane;
use crate::mesh::sphere;
use crate::render::material::Material;
use crate::render::mesh_object::MeshObject;
use crate::render::plane_object::PlaneObject;
use crate::render::sphere_object::SphereObject;

// use image::ImageBuffer;
// use image::ImageOutputFormat;
// use image::RgbImage;
// use image::codecs::hdr::HdrEncoder;
// use image::Rgb;
// use crate::math::{Color, Mat4, Quat, Vec2, Vec3};
// use crate::renderer::{Light, Renderer};
// use crate::view::View;
// use std::f64;
// use std::sync::Arc;
// use crate::bvh::BVH;
// use crate::mesh::{bunny, bunny4, cow, cube, icosahedron, Mesh, octahedron, tetrahedron};
// use crate::model::Model;
// use crate::shape::Shape;
// use crate::sphere::Sphere;
// use crate::mesh::sphere;
// use crate::transform::{Transform, TransformShape};
//
// fn main() {
//     println!("Hello, world!");
// }
//
// #[ignore]
fn test_render() {
    let size = (600, 600);
    let start = [0.0, 0.0, 1.0];
    let mut scale = Vec3::new(1.0, 1.0, 1.0);
    let mut rotation = ();
    let mut translation = Vec3::new(0.0, 0.0, 0.0);
    let mut mesh: Arc<Bvh>;
    {
        // mesh = sphere();
        // scale = Vec3::new(0.002, 0.002, 0.002);
    }
    {
        // mesh = bunny();
        // scale = Vec3::new(5.0, 5.0, 5.0);
        // translation = Vec3::new(-0.1, -0.55, 0.0);
        // rotation = Quat::from_axis_angle(Vec3::new(0.0, 1.0, 0.0), f64::consts::PI);
    }
    {
        // mesh = cow();
        // scale = Vec3::new(0.18, 0.18, 0.18);
        // translation = Vec3::new(0.0, -1.75, 0.0);
    }
    {
        // mesh = pinecone();
        // scale = Vec3::new(0.065, 0.065, 0.065);
        // translation = Vec3::new(-1.75, 0.0, 0.0);
    }
    let transform = Mat4::from_scale_rotation_translation(scale, rotation, translation);
    let transform = Transform::from(transform);
    // let shape = TransformObject::new(transform, AnyObject::Model(MeshObject::new(
    //     mesh,
    //     Material { diffuse: Color::new(1.0, 1.0, 1.0) * 0.0, dielectric: Some((1.0, 1.5)) })));
    // let mat = Mat4::from_scale_rotation_translation(Vec3::new(1.0, 1.0, 1.0), rotation, Vec3::new(0.0, 0.0, 0.0));
    // let transform = Transform::from(mat);
    let shape = TransformObject::new(transform, AnyObject::Sphere(SphereObject::new(
        Sphere::new(Vec3::new(0.0, -0.2, 0.0),
                    0.2),
        Material {
            diffuse: Color::default(),//Color::broadcast(1.0),
            dielectric: Some((1.0, 1.5)),
        },
    )));
    let plane = TransformObject::new(Transform::from(Mat4::identity()), AnyObject::Plane(PlaneObject::new(
        Vec3::new(0.0, -0.5, 0.0),
        Vec3::new(0.0, 0.0, -1.0),
        Vec3::new(-1.0, 0.0, 0.0),
        Material { diffuse: Color::new(1.0, 1.0, 1.0), dielectric: None },
        Material { diffuse: Color::new(0.1, 0.1, 0.1), dielectric: None },
    )));
    let scene = SceneObject::new(vec![shape, plane]);
    let view = View::from_fov(Vec3::from(start), Vec2::from([std::f64::consts::PI * 0.5, std::f64::consts::PI * 0.5]));
    let lightz = 1.0;
    let intensity = 10.0;
    let lightdis = 1.0;
    let lights = vec![
        Light { sphere: Sphere::new(Vec3::from([lightdis, lightdis, lightz]), 1.0), color: Color::from([0.0, 0.0, intensity]) },
        // Light { sphere: Sphere::new(Vec3::from([lightdis, -lightdis, lightz]), 1.0), color: Color::from([intensity, 0.0, 0.0]) },
        Light { sphere: Sphere::new(Vec3::from([-lightdis, lightdis, lightz]), 1.0), color: Color::from([0.0, intensity, 0.0]) },
        // Light { sphere: Sphere::new(Vec3::from([-lightdis, -lightdis, lightz]), 1.0), color: Color::from([intensity, intensity, 0.0]) },
    ];
    let mut renderer = Renderer::new(view, size, scene, lights);
    renderer.render();
    renderer.write("main");
}

fn main() {
    ThreadPoolBuilder::new().num_threads(8).build_global().unwrap();
    test_render();
    // window::show();
}