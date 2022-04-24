use rand::{Rng, thread_rng};
use crate::math::scalar::{Der, Scalar};
use crate::{Vec2, Vec3};

#[derive(Debug)]
pub struct Dielectric<T> {
    pub reflectance: T,
    pub reflect: Vec3<T>,
    pub refract: Option<Vec3<T>>,
    pub debug: T,
}

impl<T: Scalar> Dielectric<T> {
    pub fn new(inc: Vec3<T>, signed_norm: Vec3<T>, n1: T, n2: T) -> Self {
        let inc = inc.normalize();
        let dot = inc.dot(signed_norm);
        let norm;
        let cos_theta_i;
        let n_i;
        let n_t;
        if dot < T::from(0.0) {
            norm = signed_norm;
            cos_theta_i = -dot;
            n_i = n1;
            n_t = n2;
        } else {
            norm = -signed_norm;
            cos_theta_i = dot;
            n_i = n2;
            n_t = n1;
        }
        let eta = n_i / n_t;
        let sin2_theta_i = (T::from(1.0) - cos_theta_i * cos_theta_i).maximum(T::from(0.0));
        let sin2_theta_t = eta * eta * sin2_theta_i;
        let refract;
        let reflectance;
        if sin2_theta_t >= T::from(1.0) {
            refract = None;
            reflectance = T::from(1.0);
        } else {
            let cos_theta_t = (T::from(1.0) - sin2_theta_t).sqrt();
            refract = Some(inc * eta + norm * (eta * cos_theta_i - cos_theta_t));
            let r_par = ((n_t * cos_theta_i) - (n_i * cos_theta_t)) /
                ((n_t * cos_theta_i) + (n_i * cos_theta_t));
            let r_perp = ((n_i * cos_theta_i) - (n_t * cos_theta_t)) /
                ((n_i * cos_theta_i) + (n_t * cos_theta_t));
            reflectance = (r_par * r_par + r_perp * r_perp) / T::from(2.0);
        }
        let reflect = inc - norm * T::from(2.0) * inc.dot(norm);
        Dielectric { reflectance, reflect, refract, debug: dot }
    }
}

#[test]
fn test_dielectric() {
    for x in 0..10 {
        fn rand_normal() -> Vec3<f64> {
            Vec3::<()>::default().map(|_| thread_rng().gen_range(-1.0..1.0)).normalize()
        }
        let inc = rand_normal();
        let norm = rand_normal().cast();
        let dz = 0.00000001;
        let diel1 = Dielectric::new(inc.as_input(), norm, Der::from(1.0), Der::from(1.5));
        let diel2 = Dielectric::new(Vec3::new(inc.x(), inc.y(), inc.z() + dz).as_input(), norm, Der::from(1.0), Der::from(1.5));
        if let Some(r1) = Some(diel1.debug) {
            if let Some(r2) = Some(diel2.debug) {
                let dr = (r2 - r1).v;
                println!("{:?} {:?} {:?}", dr / dz, r1.d[2], r2.d[2]);
            }
        }
        // if let Some(r1) = diel1.refract {
        //     if let Some(r2) = diel2.refract {
        //         let dr = (r2 - r1).map(|x| x.v);
        //         println!("{:?}", ((dr / dz).x() - r1.der(2).x()) / (dr / dz).x());
        //     }
        // }
    }
}

// #[test]
// fn test_scalar() {
//     for x in 0..10 {
//         fn foo<T: Scalar>(v: Vec2<T>) -> T {
//             v.x().abs().sqrt() + v.y().abs().sqrt() * T::from(2.0)
//         }
//         let x = thread_rng().gen_range(0.0..1.0);
//         let y = thread_rng().gen_range(0.0..1.0);
//         let dy = 0.00000001;
//         let r1 = foo(Vec2::new(x, y).as_input());
//         let r2 = foo(Vec2::new(x, y + dy).as_input());
//         let dn = (r2 - r1).v / dy;
//         let da = r1.d[1];
//         println!("{:?} {:?}", dn, da);
//         println!("{:?}", 2.0 * (dn - da) / (dn + da));
//     }
// }