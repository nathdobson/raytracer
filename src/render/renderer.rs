use std::collections::HashMap;
use std::default::default;
use std::f64::consts::PI;
use crate::geo::view::View;
use std::path::Path;
use std::fs::File;
use std::io::BufWriter;
use std::iter;
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::time::{Duration, Instant};
use image::ImageBuffer;
use image::Rgb;
use image::codecs::hdr::HdrEncoder;
use itertools::Itertools;
use crate::render::image::ImageBuilder;
use crate::geo::ray::Ray;
use crate::math::vec::{Vec2, Vec3};
use crate::util::itertools2::Itertools2;
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng, thread_rng};
use crate::math::mat::Mat2;
use crate::geo::sphere::{Sphere, ZenithY};
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::IndexedParallelIterator;
use rayon::iter::ParallelIterator;
use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelBridge;
use indicatif::ProgressBar;
use indicatif::ProgressFinish;
use indicatif::ProgressStyle;
use crate::math::scalar::{Der, Scalar};
use crate::tree::kd_tree::{KdEntry, KdTree};
use crate::render::object::{Manifold, Object, RaycastPoint};
use crate::util::rayon::IndexedParallelIteratorExt;
use crate::geo::color::Color;
use crate::render::dielectric::Dielectric;

#[derive(Debug)]
pub struct Light {
    pub sphere: Sphere,
    pub color: Color,
}

pub struct Scene<S> {
    pub size: (usize, usize),
    pub view: View,
    pub lights: Vec<Light>,
    pub scene_object: S,
    pub photon_count: usize,
    pub photon_samples: usize,
    pub newton_steps: usize,
    pub newton_epsilon: f64,
}

pub struct Renderer<S> {
    photons: KdTree<Photon>,
    radiosity: ImageBuilder,
    perf: ImageBuilder,
    rng: SmallRng,
    scene: Scene<S>,
}

#[derive(Debug)]
pub struct Photon {
    origin: Vec3<f64>,
    dir: ZenithY<f64>,
    light: Color,
    manifold: Vec<Manifold>,
    modes: Vec<SpecularMode>,
    light_index: usize,
}

pub struct AdjustedPhoton {
    light: Color,
    position: Vec3<Der<2>>,
    normal: Vec3<f64>,
}

#[derive(Default)]
pub struct RenderedRay {
    radiosity: Color,
    depth: f64,
}

pub struct RenderedPixel {
    pos: (usize, usize),
    time: Duration,
    rendered_ray: RenderedRay,
}

#[derive(Copy, Clone, Debug, Eq, Ord, PartialOrd, PartialEq)]
pub enum SpecularMode {
    Reflect,
    Refract,
}

#[derive(Debug)]
pub struct SpecularPath<T> {
    raycast_point: RaycastPoint<T>,
    manifolds: Vec<Manifold>,
    modes: Vec<SpecularMode>,
    attenuation: T,
}

impl<S: Object> Renderer<S> {
    pub fn new(scene: Scene<S>) -> Self {
        Renderer {
            photons: KdTree::default(),
            radiosity: ImageBuilder::new(scene.size),
            perf: ImageBuilder::new(scene.size),
            scene,
            rng: SmallRng::from_entropy(),
        }
    }
    pub fn render(&mut self) {
        let photon_sources = self.scene.lights.iter().enumerate().flat_map(|(index, light)| {
            Sphere::fibonacci_sphere(self.scene.photon_count, &mut self.rng).into_iter().map(move |dir| (index, light, dir))
        }).collect::<Vec<_>>();
        let photons =
            photon_sources.par_iter()
                .progress_as("photons")
                .flat_map(|(light_index, light, dir)| {
                    let ray = Ray::new(light.sphere.orig(), dir.into_normal());
                    self.raytrace_all_specular(&ray, &[], None).into_iter().flat_map(|path| {
                        let pos = path.raycast_point.position;
                        if path.modes.len() == 0 {
                            return None;
                        }
                        Some(KdEntry::new(pos, Photon {
                            origin: ray.orig(),
                            dir: *dir,
                            light: light.color * path.attenuation,
                            manifold: path.manifolds,
                            modes: path.modes,
                            light_index: *light_index,
                        }))
                    }).collect::<Vec<_>>()
                }).collect::<Vec<_>>();
        dbg!(photons.len());
        self.photons = KdTree::new(photons);
        let mut pixels = vec![];
        for x in 0..self.scene.size.0 {
            for y in 0..self.scene.size.1 {
                pixels.push((x, y));
            }
        }
        let pixels = pixels.into_par_iter()
            .progress_as("raytrace")
            .map(|(x, y)| self.render_pixel(x, y)).collect::<Vec<_>>();
        for RenderedPixel { pos: (x, y), rendered_ray: RenderedRay { radiosity, depth }, time } in pixels {
            self.radiosity.insert(x, y, radiosity);
            self.perf.insert(x, y, Color::new(1.0, 1.0, 1.0) * time.as_secs_f64() * 3000.0);
        }
    }
    pub fn render_pixel(&self, x: usize, y: usize) -> RenderedPixel {
        let start = Instant::now();
        let sx = (x as f64 - (self.scene.size.0 as f64 - 1.0) / 2.0) / (self.scene.size.0 as f64);
        let sy = -(y as f64 - (self.scene.size.1 as f64 - 1.0) / 2.0) / (self.scene.size.1 as f64);
        let s = Vec2::from([sx, sy]);

        let rr = if true {
            self.raytrace_pixel(s)
        } else {
            RenderedRay {
                radiosity: if x % 2 == 0 {
                    Color::new(1.0 + sy * 2.0, 1.0 + sy * 2.0, 1.0 + sy * 2.0) * 0.01
                } else {
                    Color::new(1.0 - sy * 2.0, 1.0 - sy * 2.0, 1.0 - sy * 2.0) * 0.01
                },
                depth: 0.0,
            }
        };

        let end = start.elapsed();
        RenderedPixel {
            pos: (x, y),
            time: end,
            rendered_ray: rr,
        }
    }
    pub fn compute_ambient_irrad(&self, p: &RaycastPoint<f64>) -> Color {
        Color::broadcast(0.00)
    }
    pub fn compute_direct_irrad(&self, p: &RaycastPoint<f64>) -> Color {
        //let mut lighting = Color::default();
        let mut lighting = Color::new(1.0, 1.0, 1.0) * 0.0;
        for light in self.scene.lights.iter() {
            let disp = light.sphere.orig() - p.position;
            let dis2 = disp.dot(disp);
            let dis = dis2.sqrt();
            let dir = disp / dis;
            let mut dot = dir.dot(p.inter_normal);
            if dot < 0.0 {
                continue;
            }
            let ray = Ray::new_bounce(p.position, dir);
            if let Some(occlude) = self.scene.scene_object.raycast(&ray, None) {
                if occlude.time < dis {
                    continue;
                }
            }
            lighting += light.color * dot / (4.0 * PI * dis2);
        }
        lighting
    }
    pub fn compute_indirect_irrad(&self, p: &RaycastPoint<f64>) -> Color {
        let mut total = Color::default();
        let mut photons = HashMap::new();
        for photon in self.photons.nearest(&p.position, self.scene.photon_samples) {
            let photon = photon.entry;
            let mut dir = photon.value().dir;
            let mut filter_manifolds: Vec<_> = photon.value().manifold.iter().cloned().map(Some).collect();
            filter_manifolds.push(Some(p.manifold));
            let mut seps = vec![];
            for _ in 0..self.scene.newton_steps {
                let ray = Ray::new(photon.value().origin.cast(), dir.as_input().into_normal());
                let hit = self.raytrace_all_specular::<Der<2>>(&ray, &filter_manifolds, Some(&photon.value().modes));
                assert!(hit.len() < 2);
                let hit = match hit.into_iter().next() {
                    None => break,
                    Some(hit) => hit,
                };
                let sep = hit.raycast_point.manifold_point - p.manifold_point.cast();
                seps.push(sep);
                let sep_value: Vec2<f64> = sep.cast();
                let sep_jacobian: Mat2<f64> = sep.jacobian();
                let new_dir = ZenithY(dir.0 - sep_jacobian.inverse() * sep_value);
                dir = new_dir;
            }
            // let dy = 0.000001;
            // let a = self.raytrace_all_specular::<Der<2>>(
            //     &Ray::new(photon.value().origin.cast(), ZenithY(Vec2::new(dir.0.x(), dir.0.y())).as_input().into_normal()),
            //     &filter_manifolds, Some(&photon.value().modes)).into_iter().next();
            // let b = self.raytrace_all_specular::<Der<2>>(
            //     &Ray::new(photon.value().origin.cast(), ZenithY(Vec2::new(dir.0.x(), dir.0.y() + dy)).as_input().into_normal()),
            //     &filter_manifolds, Some(&photon.value().modes)).into_iter().next();
            // if let (Some(a), Some(b)) = (a, b) {
            //     let dp = b.raycast_point.manifold_point - a.raycast_point.manifold_point;
            //     let edp = a.raycast_point.manifold_point.der(1) * dy;
            //     println!("{:?} {:?} {:?}", photon.value().modes.len(), Vec2::new(dp.x().v, dp.y().v), edp);
            // }

            // if photon.value().modes.len() > 0 {
            //     println!("{:?}", seps);
            // }
            let ray = Ray::new(photon.value().origin.cast(), dir.as_input().into_normal());
            let real_photon = self.raytrace_all_specular::<Der<2>>(&ray, &[], Some(&photon.value().modes));
            assert!(real_photon.len() < 2);
            if let Some(real_photon) = real_photon.into_iter().next() {
                if real_photon.raycast_point.position.cast().distance(p.position) < self.scene.newton_epsilon {
                    photons.insert(photon.value().light_index, AdjustedPhoton {
                        light: photon.value().light,
                        position: real_photon.raycast_point.position,
                        normal: real_photon.raycast_point.inter_normal.cast(),
                    });
                }
            }
        }
        for photon in photons.values() {
            let area_vector = photon.position.der(0).cross(photon.position.der(1));
            let area = 4.0 * PI * area_vector.length();
            total += photon.light / area;
        }
        return total;
    }
    pub fn raytrace_pixel(&self, s: Vec2<f64>) -> RenderedRay {
        let ray = self.scene.view.get_ray(s);
        let mut total = Color::default();
        for path in self.raytrace_all_specular(&ray, &[], None) {
            let irrad =
                self.compute_indirect_irrad(&path.raycast_point)
                    + self.compute_direct_irrad(&path.raycast_point)
                    + self.compute_ambient_irrad(&path.raycast_point);

            total += irrad
                .map_mul(path.raycast_point.material.diffuse)
                * path.attenuation;
        }
        RenderedRay {
            radiosity: total,
            depth: 0.0,
        }
    }
    pub fn raytrace_all_specular<T: Scalar>(&self, ray: &Ray<T>, manifolds: &[Option<Manifold>], modes: Option<&[SpecularMode]>) -> Vec<SpecularPath<T>> {
        let mut output = vec![];
        self.raytrace_all_specular_rec(
            ray,
            T::from(1.0),
            manifolds,
            modes,
            &mut vec![],
            &mut vec![],
            &mut output);
        output
    }
    pub fn raytrace_all_specular_rec<T: Scalar>(
        &self,
        ray: &Ray<T>,
        attenuation: T,
        filter_manifolds: &[Option<Manifold>],
        filter_modes: Option<&[SpecularMode]>,
        output_manifolds: &mut Vec<Manifold>,
        output_modes: &mut Vec<SpecularMode>,
        output: &mut Vec<SpecularPath<T>>) {
        fn slice_pop<T: Copy>(x: &[T]) -> (Option<T>, &[T]) {
            if x.len() >= 1 {
                (x.first().cloned(), &x[1..])
            } else {
                (None, &[])
            }
        }
        if output_manifolds.len() >= 4 {
            return;
        }
        let (filter_manifold, filter_manifolds) = slice_pop(filter_manifolds);
        let first = self.scene.scene_object.raycast(ray, filter_manifold.flatten());
        let first = match first {
            None => return,
            Some(first) => first,
        };
        if filter_modes.map_or(true, |filter_modes| filter_modes.is_empty()) {
            output.push(SpecularPath {
                raycast_point: first.clone(),
                manifolds: output_manifolds.clone(),
                modes: output_modes.clone(),
                attenuation,
            });
        }
        if let Some((n1, n2)) = first.material.dielectric {
            let dielectric = Dielectric::new(ray.dir(), first.geo_normal, T::from(n1), T::from(n2));
            let (filter_mode, filter_modes) = match filter_modes {
                None => (None, None),
                Some(xs) => {
                    if xs.len() == 0 {
                        return;
                    } else {
                        (xs.first().cloned(), Some(&xs[1..]))
                    }
                }
            };
            {
                if true {
                    if filter_mode.map_or(true, |x| x == SpecularMode::Reflect) {
                        output_manifolds.push(first.manifold);
                        output_modes.push(SpecularMode::Reflect);
                        let reflect = Ray::new_bounce(first.position, dielectric.reflect);
                        self.raytrace_all_specular_rec(
                            &reflect,
                            attenuation * dielectric.reflectance,
                            filter_manifolds,
                            filter_modes,
                            output_manifolds,
                            output_modes,
                            output);
                        output_manifolds.pop();
                        output_modes.pop();
                    }
                }
            }
            if let Some(refract) = dielectric.refract {
                if filter_mode.map_or(true, |x| x == SpecularMode::Refract) {
                    output_manifolds.push(first.manifold);
                    output_modes.push(SpecularMode::Refract);
                    let reflect = Ray::new_bounce(first.position, refract);
                    self.raytrace_all_specular_rec(
                        &reflect,
                        attenuation * (T::from(1.0) - dielectric.reflectance),
                        filter_manifolds,
                        filter_modes,
                        output_manifolds,
                        output_modes,
                        output);
                    output_manifolds.pop();
                    output_modes.pop();
                }
            }
        }
    }
    pub fn images(&self) -> HashMap<String, Vec<u8>> {
        let mut result = HashMap::new();
        result.insert("image.hdr".to_string(), self.radiosity.to_hdr());
        result.insert("image.pq.hdr".to_string(), self.radiosity.smpte2048_encode().to_hdr());
        result.insert("perf.hdr".to_string(), self.perf.to_hdr());
        result
    }
}
