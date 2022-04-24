// use std::{iter, slice};
// use std::ops::{Index, IndexMut};
//
// use itertools::Itertools;
//
// pub type Vec2 = DVec2;
// pub type Vec3 = DVec3;
// pub type Vec4 = DVec4;
//
// pub type Mat2 = DMat2;
// pub type Mat3 = DMat3;
// pub type Mat4 = DMat4;
//
// pub type Quat = DQuat;
//
// pub type Color = DVec3;
//
// pub trait VecExt: Index<usize, Output=f64> + IndexMut<usize> + Sized {
//     const ZERO: Self;
//     fn broadcast(x: f64) -> Self;
//     fn as_slice(&self) -> &[f64];
//     fn iter(&self) -> slice::Iter<f64> { self.as_slice().iter() }
//     fn try_collect(iter: impl Iterator<Item=f64>) -> Option<Self>;
// }
//
// pub trait VecExt1Plus: VecExt {
//     fn x(&self) -> f64 { self[0] }
//     fn x_mut(&mut self) -> &mut f64 { &mut self[0] }
// }
//
// pub trait VecExt2Plus: VecExt1Plus {
//     fn y(&self) -> f64 { self[1] }
//     fn y_mut(&mut self) -> &mut f64 { &mut self[1] }
// }
//
// pub trait VecExt3Plus: VecExt2Plus {
//     fn z(&self) -> f64 { self[2] }
//     fn z_mut(&mut self) -> &mut f64 { &mut self[2] }
// }
//
// pub trait VecExt4Plus: VecExt3Plus {
//     fn w(&self) -> f64 { self[3] }
//     fn w_mut(&mut self) -> &mut f64 { &mut self[3] }
// }
//
// impl VecExt for Vec2 {
//     const ZERO: Self = Vec2::ZERO;
//     fn broadcast(x: f64) -> Self { Self::splat(x) }
//     fn as_slice(&self) -> &[f64] { self.as_ref() }
//     fn try_collect(iter: impl Iterator<Item=f64>) -> Option<Self> {
//         let (x, y) = iter.collect_tuple()?;
//         Some(Vec2::new(x, y))
//     }
// }
//
// impl VecExt for Vec3 {
//     const ZERO: Self = Vec3::ZERO;
//     fn broadcast(x: f64) -> Self { Self::splat(x) }
//     fn as_slice(&self) -> &[f64] { self.as_ref() }
//     fn try_collect(iter: impl Iterator<Item=f64>) -> Option<Self> {
//         let (x, y, z) = iter.collect_tuple()?;
//         Some(Vec3::new(x, y, z))
//     }
// }
//
// impl VecExt for Vec4 {
//     const ZERO: Self = Vec4::ZERO;
//     fn broadcast(x: f64) -> Self { Self::splat(x) }
//     fn as_slice(&self) -> &[f64] { self.as_ref() }
//     fn try_collect(iter: impl Iterator<Item=f64>) -> Option<Self> {
//         let (x, y, z, w) = iter.collect_tuple()?;
//         Some(Vec4::new(x, y, z, w))
//     }
// }
//
// impl VecExt1Plus for Vec2 {}
//
// impl VecExt1Plus for Vec3 {}
//
// impl VecExt1Plus for Vec4 {}
//
// impl VecExt2Plus for Vec2 {}
//
// impl VecExt2Plus for Vec3 {}
//
// impl VecExt2Plus for Vec4 {}
//
// impl VecExt3Plus for Vec3 {}
//
// impl VecExt3Plus for Vec4 {}
//
// impl VecExt4Plus for Vec4 {}
//
// // pub trait VecExt {
// //     const LENGTH: usize;
// //     type Item;
// // fn concat<const L2: usize>(self, other: Vector<Self::Item, L2>) -> Vector<Self::Item, { Self::LENGTH + L2 }>;
// // fn append(self, other: Self::Item) -> Vector<Self::Item, { Self::LENGTH + 1 }>;
// // fn truncate<const L2: usize>(self) -> Vector<Self::Item, L2>;
// // }
//
// // impl<T, const L: usize> VecExt for Vector<T, L> {
// //     const LENGTH: usize = L;
// //     type Item = T;
// //     fn concat<const L2: usize>(self, other: Vector<Self::Item, L2>) -> Vector<Self::Item, { Self::LENGTH + L2 }> {
// //         Vector::try_from_iter(self.into_iter().chain(other.into_iter())).unwrap()
// //     }
// //     fn append(self, other: Self::Item) -> Vector<Self::Item, { Self::LENGTH + 1 }> {
// //         Vector::try_from_iter(self.into_iter().chain(iter::once(other))).unwrap()
// //     }
// //     fn truncate<const L2: usize>(self) -> Vector<Self::Item, L2> {
// //         Vector::try_from_iter(self.into_iter().take(L2)).unwrap()
// //     }
// // }
pub mod mat;
pub mod quat;
pub mod scalar;
// mod trispline;
pub mod vec;
pub mod scalar_key;
