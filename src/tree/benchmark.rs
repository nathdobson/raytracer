extern crate test;

use std::hint::black_box;
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};
use crate::{pinecone, Vec3};
use crate::geo::ray::Ray;
use crate::tree::kd_tree::{KdEntry, KdTree};

#[bench]
#[ignore]
fn bench_mesh(b: &mut test::Bencher) {
    let mesh = pinecone();
    let bounds = mesh.bounds();
    let mut rng = SmallRng::seed_from_u64(10212233);
    b.iter(|| {
        let orig = rng.sample(&bounds);
        let mut dir;
        loop {
            dir = Vec3::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0));
            if dir.length() > 1.0 { continue; }
            dir = dir.normalize();
            break;
        }
        let ray = Ray::new(orig, dir);
        black_box(mesh.raycast(&ray, None));
    });
}

#[bench]
#[ignore]
fn bench_kd_tree(b: &mut test::Bencher) {
    let mut rng = SmallRng::seed_from_u64(10212233);
    let tree = KdTree::new((0..100000).map(|_| {
        KdEntry::new(Vec3::<()>::default().map(|x| rng.gen_range(0.0..1.0)), ())
    }).collect());
    let delta = 1.0;
    b.iter(|| {
        tree.nearest(&Vec3::<()>::default().map(|x| rng.gen_range(0.0 - delta..1.0 + delta)), 1)
    });
}