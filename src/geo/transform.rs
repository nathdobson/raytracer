use std::ops::{Mul, Neg};
use crate::geo::color::Color;
use crate::math::mat::{Mat3, Mat4};
use crate::math::scalar::Scalar;
use crate::geo::ray::Ray;
use crate::math::vec::Vec3;

// #[derive(Copy, Clone, Debug)]
// pub struct Translate(pub Vec3);
//
// #[derive(Copy, Clone, Debug)]
// pub struct Scale(pub Vec3);

// #[derive(Copy, Clone, Debug)]
// pub struct Yaw(pub f64);
//
// #[derive(Copy, Clone, Debug)]
// pub struct Pitch(pub f64);
//
// #[derive(Copy, Clone, Debug)]
// pub struct Roll(pub f64);

// #[derive(Copy, Clone, Debug)]
// pub struct Rotate {
//     yaw: f64,
//     pitch: f64,
//     roll: f64,
// }

// #[derive(Copy, Clone, Debug)]
// pub struct TransformParts {
//     translate: Translate,
//     scale: Scale,
//     rotate: Rotate,
// }

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Transform<T> {
    forward_norm: Mat3<T>,
    forward: Mat4<T>,
    reverse_norm: Mat3<T>,
    reverse: Mat4<T>,
}

impl<T: Scalar> From<Mat4<T>> for Transform<T> {
    fn from(forward: Mat4<T>) -> Self {
        let reverse: Mat4<T> = forward.inverse();
        let reverse_norm: Mat3<T> = Mat3::from_mat4(&forward).transpose();
        let forward_norm: Mat3<T> = reverse_norm.inverse();
        Transform { forward_norm, forward, reverse_norm, reverse }
    }
}

impl<T: Scalar> Transform<T> {
    pub fn cast<T2: From<T>>(&self) -> Transform<T2> {
        Transform {
            forward_norm: self.forward_norm.cast(),
            forward: self.forward.cast(),
            reverse_norm: self.reverse_norm.cast(),
            reverse: self.reverse.cast(),
        }
    }
    pub fn reverse_pos(&self, pos: Vec3<T>) -> Vec3<T> {
        self.reverse.transform_position(pos)
    }
    pub fn reverse_tang(&self, tang: Vec3<T>) -> Vec3<T> {
        self.reverse.transform_tangent(tang)
    }
    pub fn reverse_ray(&self, ray: &Ray<T>) -> Ray<T> {
        Ray::new(self.reverse_pos(ray.orig()), self.reverse_tang(ray.dir()))
    }
    pub fn forward_norm(&self, norm: Vec3<T>) -> Vec3<T> {
        (self.forward_norm * norm).normalize()
    }
}


// pub fn affinize(m: Mat3) -> Mat4 {
//     Mat4::from_mat3(m)
// }
//
// pub fn linearize(m: Mat4) -> Mat3 {
//     Mat3::from_mat4(m)
// }
//
// impl From<Translate> for Mat4 {
//     fn from(t: Translate) -> Self {
//         Mat4::from_translation(t.0)
//         // Mat4::from([
//         //     [1.0, 0.0, 0.0, t.0.x()],
//         //     [0.0, 1.0, 0.0, t.0.y()],
//         //     [0.0, 0.0, 1.0, t.0.z()],
//         //     [0.0, 0.0, 0.0, 1.0],
//         // ])
//     }
// }
//
// impl From<Scale> for Mat3 {
//     fn from(s: Scale) -> Self {
//         //Mat3::diagonal(s.0)
//         Mat3::from_diagonal(s.0)
//     }
// }
//
// impl From<Scale> for Mat4 { fn from(s: Scale) -> Self { affinize(s.into()) } }
//
// // impl From<Yaw> for Mat3 {
// //     fn from(y: Yaw) -> Self {
// //         Mat3::from([
// //             [y.0.cos(), -y.0.sin(), 0.0],
// //             [y.0.sin(), y.0.cos(), 0.0],
// //             [0.0, 0.0, 1.0],
// //         ])
// //     }
// // }
// //
// // impl From<Yaw> for Mat4 { fn from(y: Yaw) -> Self { affinize(y.into()) } }
// //
// // impl From<Pitch> for Mat4 { fn from(p: Pitch) -> Self { affinize(p.into()) } }
// //
// // impl From<Pitch> for Mat3 {
// //     fn from(p: Pitch) -> Self {
// //         Mat3::from([
// //             [p.0.cos(), 0.0, p.0.sin()],
// //             [0.0, 1.0, 0.0],
// //             [-p.0.sin(), 0.0, p.0.cos()],
// //         ])
// //     }
// // }
// //
// // impl From<Roll> for Mat3 {
// //     fn from(r: Roll) -> Self {
// //         Mat3::from([
// //             [1.0, 0.0, 0.0],
// //             [0.0, r.0.cos(), -r.0.sin()],
// //             [0.0, r.0.sin(), r.0.cos()],
// //         ])
// //     }
// // }
// //
// // impl From<Roll> for Mat4 { fn from(r: Roll) -> Self { affinize(r.into()) } }
//
// impl From<Rotate> for Mat3 {
//     fn from(r: Rotate) -> Self {
//
//         //Mat3::from(r.yaw) * Mat3::from(r.pitch) * Mat3::from(r.roll)
//     }
// }
//
// impl From<Rotate> for Mat4 { fn from(r: Rotate) -> Self { affinize(r.into()) } }
//
// impl From<TransformParts> for Mat4 {
//     fn from(p: TransformParts) -> Self {
//         let a = Mat4::from(p.scale) * Mat4::from(p.rotate);
//         let b = Mat4::from(p.translate);
//         let c = a * b;
//         println!("{:?}", b * b);
//         c
//     }
// }
//
// impl Neg for Translate {
//     type Output = Self;
//     fn neg(self) -> Self::Output { Translate(-self.0) }
// }
//
// impl Neg for Scale {
//     type Output = Self;
//     fn neg(self) -> Self::Output { Scale(self.0.recip()) }
// }
//
// // impl Neg for Yaw {
// //     type Output = Self;
// //     fn neg(self) -> Self::Output { Yaw(-self.0) }
// // }
// //
// // impl Neg for Pitch {
// //     type Output = Self;
// //     fn neg(self) -> Self::Output { Pitch(-self.0) }
// // }
// //
// // impl Neg for Roll {
// //     type Output = Self;
// //     fn neg(self) -> Self::Output { Roll(-self.0) }
// // }
//
//
// impl Transform {
//     fn from_into_mat4<T>(x: T) -> Self where Mat4: From<T>, T: Copy {
//         Transform::from(Mat4::from(x))
//     }
// }
//
// impl From<Translate> for Transform { fn from(t: Translate) -> Self { Transform::from_into_mat4(t) } }
//
// impl From<Scale> for Transform { fn from(s: Scale) -> Self { Transform::from_into_mat4(s) } }
//
// // impl From<Yaw> for Transform { fn from(y: Yaw) -> Self { Transform::from_into_mat4(y) } }
// //
// // impl From<Pitch> for Transform { fn from(p: Pitch) -> Self { Transform::from_into_mat4(p) } }
// //
// // impl From<Roll> for Transform { fn from(r: Roll) -> Self { Transform::from_into_mat4(r) } }
//
// impl From<Rotate> for Transform { fn from(r: Rotate) -> Self { Transform::from_into_mat4(r) } }
//
// impl From<TransformParts> for Transform { fn from(t: TransformParts) -> Self { Transform::from_into_mat4(t) } }
//
// impl Mul for Transform {
//     type Output = Self;
//     fn mul(self, rhs: Self) -> Self::Output {
//         Transform {
//             forward_norm: self.forward_norm * rhs.forward_norm,
//             forward: self.forward * rhs.forward,
//             reverse_norm: rhs.reverse_norm * self.reverse_norm,
//             reverse: rhs.reverse * self.reverse,
//         }
//     }
// }
//
// impl Transform {
//     fn one() -> Self {
//         Transform {
//             forward_norm: Mat3::one(),
//             forward: Mat4::one(),
//             reverse_norm: Mat3::one(),
//             reverse: Mat4::one(),
//         }
//     }
// }
//
// #[test]
// fn test_one() {
//     let transform: Transform = TransformParts {
//         translate: Translate(0.0.broadcast()),
//         scale: Scale(1.0.broadcast()),
//         rotate: Rotate {
//             yaw: 0.0,
//             pitch: 0.0,
//             roll: 0.0,
//         },
//     }.into();
//     assert_eq!(transform, Transform::one());
// }
//
// #[test]
// fn test_translate() {
//     let transform: Transform = TransformParts {
//         translate: Translate(Vec3::from([1.0, 2.0, 3.0])),
//         scale: Scale(1.0.broadcast()),
//         rotate: Rotate {
//             yaw: 0.0,
//             pitch: 0.0,
//             roll: 0.0,
//         },
//     }.into();
//     assert_eq!(transform, Transform {
//         forward_norm: Mat3::one(),
//         forward: Mat4::from([
//             [1.0, 0.0, 0.0, 1.0],
//             [0.0, 1.0, 0.0, 2.0],
//             [0.0, 0.0, 1.0, 4.0],
//             [0.0, 0.0, 0.0, 1.0],
//         ]),
//         reverse_norm: Mat3::one(),
//         reverse: Mat4::from([
//             [1.0, 0.0, 0.0, -1.0],
//             [0.0, 1.0, 0.0, -2.0],
//             [0.0, 0.0, 1.0, -4.0],
//             [0.0, 0.0, 0.0, 1.0],
//         ]),
//     });
// }