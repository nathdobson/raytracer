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

use std::default::default;
use std::f64::consts::PI;
use std::sync::Arc;
use crate::geo::color::Color;
use crate::geo::sphere::Sphere;
use crate::geo::transform::{Transform, TransformBuilder};
use crate::geo::view::View;
use crate::math::mat::Mat4;
use crate::math::vec::{Vec2, Vec3};
use crate::mesh::{bunny, cow, pinecone, sphere};
use crate::render::any_object::AnyObject;
use crate::render::material::Material;
use crate::render::mesh_object::MeshObject;
use crate::render::plane_object::PlaneObject;
use crate::render::renderer::{Light, Renderer, Scene};
use crate::render::scene_object::SceneObject;
use crate::render::sphere_object::SphereObject;
use crate::render::transform_object::TransformObject;
use crate::tree::bvh::Bvh;

pub mod util;
pub mod math;
pub mod render;
pub mod geo;
pub mod mesh;
pub mod tree;

pub struct SceneBuilder {
    pub time: usize,
}

impl SceneBuilder {
    pub fn material(&self) -> Material {
        Material { diffuse: Color::new(1.0, 1.0, 1.0) * 0.0, dielectric: Some((1.0, 1.5)) }
    }
    pub fn make_mesh(&self, mesh: Arc<Bvh>, transform: Transform<f64>) -> TransformObject<AnyObject> {
        TransformObject::new(
            transform,
            AnyObject::Model(MeshObject::new(mesh, self.material())))
    }
    pub fn sphere_mesh(&self) -> TransformObject<AnyObject> {
        self.make_mesh(sphere(), TransformBuilder::new().scale(0.002).build())
    }
    pub fn bunny(&self) -> TransformObject<AnyObject> {
        // rotation = Quat::from_axis_angle(Vec3::new(0.0, 1.0, 0.0), f64::consts::PI);
        self.make_mesh(bunny(), TransformBuilder::new().scale(5.0).translate(-0.1, -0.55, 0.0).rotate(()).build())
    }
    pub fn cow(&self) -> TransformObject<AnyObject> {
        self.make_mesh(cow(), TransformBuilder::new().scale(0.18).translate(0.0, -1.75, 0.0).rotate(()).build())
    }
    pub fn pinecone(&self) -> TransformObject<AnyObject> {
        self.make_mesh(pinecone(), TransformBuilder::new().scale(0.065).translate(-1.75, 0.0, 0.0).rotate(()).build())
    }
    pub fn sphere(&self) -> TransformObject<AnyObject> {
        TransformObject::new(Transform::default(), AnyObject::Sphere(SphereObject::new(
            Sphere::new(Vec3::new(0.0, -0.2, 0.0),
                        0.2),
            Material {
                diffuse: default(),
                dielectric: Some((1.0, 1.5)),
            },
        )))
    }
    pub fn plane(&self) -> TransformObject<AnyObject> {
        TransformObject::new(Transform::from(Mat4::identity()), AnyObject::Plane(PlaneObject::new(
            Vec3::new(0.0, -0.5, 0.0),
            Vec3::new(0.0, 0.0, -1.0),
            Vec3::new(-1.0, 0.0, 0.0),
            Material { diffuse: Color::new(1.0, 1.0, 1.0), dielectric: None },
            Material { diffuse: Color::new(0.1, 0.1, 0.1), dielectric: None },
        )))
    }
    pub fn view(&self) -> View {
        View::from_fov(Vec3::new(0.0, 0.0, 1.0), Vec2::from([std::f64::consts::PI * 0.5, std::f64::consts::PI * 0.5]))
    }
    pub fn lights(&self) -> Vec<Light> {
        let lightz = 1.0;
        let intensity = 100.0;
        let lightdis = 1.0;
        let cos = (self.time as f64 / 100.0 * PI * 2.0).cos();
        let sin = (self.time as f64 / 100.0 * PI * 2.0).sin();
        vec![
            Light { sphere: Sphere::new(Vec3::from([lightdis * cos, lightdis * sin, lightz]), 1.0), color: Color::from([0.0, 0.0, intensity]) },
            // Light { sphere: Sphere::new(Vec3::from([lightdis, -lightdis, lightz]), 1.0), color: Color::from([intensity, 0.0, 0.0]) },
            Light { sphere: Sphere::new(Vec3::from([-lightdis * cos, lightdis * sin, lightz]), 1.0), color: Color::from([0.0, intensity, 0.0]) },
            // Light { sphere: Sphere::new(Vec3::from([-lightdis, -lightdis, lightz]), 1.0), color: Color::from([intensity, intensity, 0.0]) },
        ]
    }
    pub fn scene_object(&self) -> SceneObject {
        SceneObject::new(vec![
            self.sphere(),
            self.plane(),
        ])
    }
    pub fn scene(&self) -> Scene<SceneObject> {
        Scene {
            size: (300, 300),
            view: self.view(),
            lights: self.lights(),
            scene_object: self.scene_object(),
            photon_count: 10000000,
            photon_samples: 3,
            newton_steps: 5,
            newton_epsilon: 0.00001,
        }
    }
}

