use std::cmp::Ordering;
use std::default::default;
use std::fmt::Debug;
use std::iter::{Product, Sum};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, Sub, SubAssign};
use crate::math::vec::Vector;

pub trait Scalar
: From<f64>
+ Copy
+ Add<Output=Self>
+ Sub<Output=Self>
+ Mul<Output=Self>
+ Div<Output=Self>
+ Rem<Output=Self>
+ MulAssign
+ AddAssign
+ SubAssign
+ DivAssign
+ Sum
+ Product
+ PartialOrd
+ Default
+ Debug
+ roots::FloatType {
    fn minimum(self, other: Self) -> Self;
    fn maximum(self, other: Self) -> Self;
    fn is_finite(self) -> bool;
    //fn sqrt(self) -> Self;
    fn real_eq(self, other: Self) -> bool;
    fn real_cmp(self, other: Self) -> Ordering;
    fn not_nan(self) -> bool;
    fn into_const(self) -> f64;
}

impl Scalar for f64 {
    fn minimum(self, other: Self) -> Self { f64::minimum(self, other) }
    fn maximum(self, other: Self) -> Self { f64::maximum(self, other) }
    fn is_finite(self) -> bool { f64::is_finite(self) }
    //fn sqrt(self) -> Self { f64::sqrt(self) }

    fn real_eq(self, other: Self) -> bool {
        (self.is_nan() && other.is_nan()) || self == other
    }

    fn real_cmp(self, other: Self) -> Ordering {
        if self.is_nan() {
            if other.is_nan() {
                Ordering::Equal
            } else {
                Ordering::Less
            }
        } else {
            if other.is_nan() {
                Ordering::Greater
            } else {
                self.partial_cmp(&other).unwrap()
            }
        }
    }

    fn not_nan(self) -> bool { !self.is_nan() }

    fn into_const(self) -> f64 { self }
}

pub struct DerX {
    pub v: f64,
    pub d: f64,
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct Der<const N: usize> {
    pub v: f64,
    pub d: Vector<N, f64>,
}

impl<const N: usize> Der<N> {
    pub fn var(v: f64, n: usize) -> Self {
        let mut d: Vector<N, _> = default();
        d[n] = 1.0;
        Der { v, d }
    }
    fn oper1(self, v: impl FnOnce(f64) -> f64, mut d: impl FnMut(f64) -> f64) -> Self {
        Der {
            v: v(self.v),
            d: self.d.map(|d1| d1 * d(self.v)),
        }
    }
    fn oper2(self, rhs: Self, v: impl FnOnce(f64, f64) -> f64, mut d: impl FnMut(DerX, DerX) -> f64) -> Self {
        Der {
            v: v(self.v, rhs.v),
            d: self.d.zip(rhs.d).map(|(d1, d2)| d(DerX { v: self.v, d: d1 }, DerX { v: rhs.v, d: d2 })),
        }
    }
}

impl<const N: usize> Default for Der<N> {
    fn default() -> Self {
        Der { v: default(), d: default() }
    }
}

impl<const N: usize> Add for Der<N> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        self.oper2(rhs, |x, y| x + y, |x, y| x.d + y.d)
    }
}

impl<const N: usize> Sub for Der<N> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output { self.oper2(rhs, |x, y| x - y, |x, y| x.d - y.d) }
}

impl<const N: usize> Mul for Der<N> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        self.oper2(rhs, |x, y| x * y, |x, y| x.v * y.d + x.d * y.v)
    }
}

impl<const N: usize> Div for Der<N> {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        self.oper2(rhs, |x, y| x / y, |x, y| (y.v * x.d - x.v * y.d) / (y.v * y.v))
    }
}

// impl<const N: usize> Add for &Der<N> {
//     type Output = Der<N>;
//     fn add(self, rhs: Self) -> Self::Output { (*self).add(*rhs) }
// }
//
// impl<const N: usize> Add<&Der<N>> for Der<N> {
//     type Output = Der<N>;
//     fn add(self, rhs: &Der<N>) -> Self::Output { self.add(*rhs) }
// }
//
// impl<const N: usize> Add<Der<N>> for &Der<N> {
//     type Output = Der<N>;
//     fn add(self, rhs: Der<N>) -> Self::Output { (*self).add(rhs) }
// }

impl<const N: usize> AddAssign for Der<N> {
    fn add_assign(&mut self, rhs: Self) { *self = (*self) + rhs; }
}

// impl<const N: usize> AddAssign<&Der<N>> for Der<N> {
//     fn add_assign(&mut self, rhs: &Self) { *self = (*self) + rhs; }
// }
//
// impl<const N: usize> Sub for &Der<N> {
//     type Output = Der<N>;
//     fn sub(self, rhs: Self) -> Self::Output { (*self).sub(*rhs) }
// }
//
// impl<const N: usize> Sub<&Der<N>> for Der<N> {
//     type Output = Der<N>;
//     fn sub(self, rhs: &Der<N>) -> Self::Output { self.sub(*rhs) }
// }
//
// impl<const N: usize> Sub<Der<N>> for &Der<N> {
//     type Output = Der<N>;
//     fn sub(self, rhs: Der<N>) -> Self::Output { (*self).sub(rhs) }
// }

impl<const N: usize> SubAssign for Der<N> {
    fn sub_assign(&mut self, rhs: Self) { *self = (*self) - rhs; }
}

// impl<const N: usize> SubAssign<&Der<N>> for Der<N> {
//     fn sub_assign(&mut self, rhs: &Self) { *self = (*self) - rhs; }
// }

// impl<const N: usize> Mul for &Der<N> {
//     type Output = Der<N>;
//     fn mul(self, rhs: Self) -> Self::Output { (*self).mul(*rhs) }
// }
//
// impl<const N: usize> Mul<&Der<N>> for Der<N> {
//     type Output = Der<N>;
//     fn mul(self, rhs: &Der<N>) -> Self::Output { self.mul(*rhs) }
// }
//
// impl<const N: usize> Mul<Der<N>> for &Der<N> {
//     type Output = Der<N>;
//     fn mul(self, rhs: Der<N>) -> Self::Output { (*self).mul(rhs) }
// }

impl<const N: usize> MulAssign for Der<N> {
    fn mul_assign(&mut self, rhs: Self) { *self = (*self) * rhs; }
}

// impl<const N: usize> MulAssign<&Der<N>> for Der<N> {
//     fn mul_assign(&mut self, rhs: &Self) { *self = (*self) * rhs; }
// }
//
// impl<const N: usize> Div for &Der<N> {
//     type Output = Der<N>;
//     fn div(self, rhs: Self) -> Self::Output { (*self).div(*rhs) }
// }
//
// impl<const N: usize> Div<&Der<N>> for Der<N> {
//     type Output = Der<N>;
//     fn div(self, rhs: &Der<N>) -> Self::Output { self.div(*rhs) }
// }
//
// impl<const N: usize> Div<Der<N>> for &Der<N> {
//     type Output = Der<N>;
//     fn div(self, rhs: Der<N>) -> Self::Output { (*self).div(rhs) }
// }

impl<const N: usize> DivAssign for Der<N> {
    fn div_assign(&mut self, rhs: Self) { *self = (*self) / rhs; }
}

// impl<const N: usize> DivAssign<&Der<N>> for Der<N> {
//     fn div_assign(&mut self, rhs: &Self) { *self = (*self) / rhs; }
// }

impl<const N: usize> From<f64> for Der<N> {
    fn from(v: f64) -> Self { Der { v, d: default() } }
}

impl<const N: usize> roots::FloatType for Der<N> {
    fn zero() -> Self {
        Der { v: 0.0, d: Vector::broadcast(0.0) }
    }

    fn one() -> Self {
        todo!()
    }

    fn one_third() -> Self {
        todo!()
    }

    fn pi() -> Self {
        todo!()
    }

    fn two_third_pi() -> Self {
        todo!()
    }

    fn sqrt(self) -> Self {
        self.oper1(|x| x.sqrt(), |x| 1.0 / (2.0 * x.sqrt()))
    }

    fn atan(self) -> Self {
        todo!()
    }

    fn acos(self) -> Self {
        todo!()
    }

    fn sin(self) -> Self {
        self.oper1(|x| x.sin(), |x| x.cos())
    }

    fn cos(self) -> Self {
        self.oper1(|x| x.cos(), |x| -x.sin())
    }

    fn abs(self) -> Self {
        if self.v < 0.0 {
            -self
        } else {
            self
        }
    }

    fn powf(self, n: Self) -> Self {
        todo!()
    }
}

impl<const N: usize> From<i16> for Der<N> {
    fn from(x: i16) -> Self {
        Self::from(x as f64)
    }
}

impl<const N: usize> Neg for Der<N> {
    type Output = Self;
    fn neg(self) -> Self::Output {
        self.oper1(|x| -x, |x| -1.0)
    }
}

impl<const N: usize> Rem for Der<N> {
    type Output = Self;
    fn rem(self, rhs: Self) -> Self::Output {
        Der { v: self.v.rem(rhs.v), d: self.d }
    }
}

impl<const N: usize> Scalar for Der<N> {
    fn minimum(self, other: Self) -> Self {
        if self.v.is_nan() {
            self
        } else if other.v.is_nan() {
            other
        } else if self.v <= other.v {
            self
        } else {
            other
        }
    }
    fn maximum(self, other: Self) -> Self {
        if self.v.is_nan() {
            self
        } else if other.v.is_nan() {
            other
        } else if self.v <= other.v {
            other
        } else {
            self
        }
    }
    fn is_finite(self) -> bool { self.v.is_finite() }

    // fn sqrt(self) -> Self {
    //     todo!()
    // }

    fn real_eq(self, other: Self) -> bool {
        self.v.real_eq(other.v)
    }

    fn real_cmp(self, other: Self) -> Ordering {
        self.v.real_cmp(other.v)
    }

    fn not_nan(self) -> bool {
        self.v.not_nan() && self.d.into_iter().all(|x| x.not_nan())
    }

    fn into_const(self) -> f64 { self.v }
}

impl<const N: usize> Sum for Der<N> {
    fn sum<I: Iterator<Item=Self>>(iter: I) -> Self { iter.fold(Self::from(0.0), Self::add) }
}

impl<const N: usize> Product for Der<N> {
    fn product<I: Iterator<Item=Self>>(iter: I) -> Self { iter.fold(Self::from(1.0), Self::mul) }
}

impl<const N: usize> From<Der<N>> for f64 {
    fn from(x: Der<N>) -> Self { x.v }
}

type Der1 = Der<1>;
type Der2 = Der<2>;
type Der3 = Der<3>;

#[test]
fn test() {
    let x = Der2::var(1.0, 0);
    let y = Der2::var(2.0, 1);
    let c = Der2::from(3.0);
    let f = x + y + c;
    assert_eq!(f, Der { v: 6.0, d: [1.0, 1.0].into() });
}