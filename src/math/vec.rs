use std::cmp::max;
use std::fmt::{Debug, Formatter};
use std::iter::Sum;
use std::ops::{Add, AddAssign, Div, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign};
use arrayvec::ArrayVec;
use crate::math::mat::Matrix;
use crate::math::scalar::{Der, Scalar};

#[derive(Copy, Clone, PartialEq, PartialOrd)]
pub struct Vector<const N: usize, T>([T; N]);

pub type Vec2<T> = Vector<2, T>;
pub type Vec3<T> = Vector<3, T>;
pub type Vec4<T> = Vector<4, T>;

impl<const N: usize, T> Vector<N, T> {
    pub fn zip<T2>(self, other: Vector<N, T2>) -> Vector<N, (T, T2)> { Vector(self.0.zip(other.0)) }
    pub fn map<T2, F: FnMut(T) -> T2>(self, f: F) -> Vector<N, T2> { Vector(self.0.map(f)) }
    pub fn broadcast(value: T) -> Self where T: Copy { Vector([value; N]) }
    pub fn cast<T2: From<T>>(self) -> Vector<N, T2> {
        self.map(|x| x.into())
    }
}

impl<const N: usize> Vector<N, f64> {
    pub fn as_input(self) -> Vector<N, Der<N>> {
        let mut result = Vector::default();
        for i in 0..N {
            result[i] = Der::var(self[i], i)
        }
        result
    }
}

impl<const N: usize, T> Vector<N, T> where T: Scalar {
    pub fn dot(self, other: Self) -> T {
        self.zip(other).into_iter().map(|(x, y)| x * y).sum()
    }
    pub fn square_length(self) -> T {
        self.dot(self)
    }
    pub fn length(self) -> T { self.square_length().sqrt() }
    pub fn normalize(self) -> Self {
        self / self.length()
    }
    pub fn zip_mul(self, other: Self) -> Self {
        self.zip(other).map(|(x, y)| x * y)
    }
    pub fn distance(self, other: Self) -> T {
        (self - other).length()
    }
    pub fn square_distance(self, other: Self) -> T {
        (self - other).square_length()
    }
    pub fn nan() -> Self {
        Self::broadcast(T::from(f64::NAN))
    }
    pub fn clamp(self) -> Self {
        self.map(|x| x.maximum(T::from(0.0)))
    }
    pub fn map_mul(self, other: Self) -> Self {
        self.zip(other).map(|(x, y)| x * y)
    }
}

impl<const N: usize> Vector<N, Der<N>> {
    pub fn jacobian(&self) -> Matrix<N, f64> {
        Matrix::from_row_vector(self.map(|x| x.d))
    }
}

impl<const N: usize, const M: usize> Vector<M, Der<N>> {
    pub fn der(&self, n: usize) -> Vector<M, f64> {
        Vector::from(self.0.map(|x| x.d[n]))
    }
}

impl<const N: usize, T: Default> Default for Vector<N, T> {
    fn default() -> Self {
        let mut arrayvec = ArrayVec::new();
        for _ in 0..N {
            arrayvec.try_push(T::default()).unwrap();
        }
        Vector(arrayvec.into_inner().unwrap_or_else(|_| panic!()))
    }
}

impl<const N: usize, T> From<[T; N]> for Vector<N, T> { fn from(x: [T; N]) -> Self { Vector(x) } }

impl<const N: usize, T> From<Vector<N, T>> for [T; N] { fn from(x: Vector<N, T>) -> Self { x.0 } }

impl<const N: usize, T> Index<usize> for Vector<N, T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output { &self.0[index] }
}

impl<const N: usize, T> IndexMut<usize> for Vector<N, T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output { &mut self.0[index] }
}

impl<const N: usize, T> IntoIterator for Vector<N, T> {
    type Item = T;
    type IntoIter = <[T; N] as IntoIterator>::IntoIter;
    fn into_iter(self) -> Self::IntoIter { self.0.into_iter() }
}

impl<const N: usize, T: Scalar> Add for Vector<N, T> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output { self.zip(rhs).map(|(x, y)| x + y) }
}

impl<const N: usize, T: Scalar> AddAssign for Vector<N, T> {
    fn add_assign(&mut self, rhs: Self) { *self = *self + rhs; }
}

impl<const N: usize, T: Scalar> Sub for Vector<N, T> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output { self.zip(rhs).map(|(x, y)| x - y) }
}

impl<const N: usize, T: Scalar> SubAssign for Vector<N, T> {
    fn sub_assign(&mut self, rhs: Self) { *self = *self - rhs; }
}

impl<const N: usize, T: Scalar> Mul<T> for Vector<N, T> {
    type Output = Self;
    fn mul(self, rhs: T) -> Self::Output { self.map(|x| x * rhs) }
}

impl<const N: usize> Mul<Vector<N, f64>> for f64 {
    type Output = Vector<N, f64>;
    fn mul(self, rhs: Vector<N, f64>) -> Self::Output { rhs.map(|x| self * x) }
}

impl<const N1: usize, const N2: usize> Mul<Vector<N1, Der<N2>>> for Der<N2> {
    type Output = Vector<N1, Der<N2>>;
    fn mul(self, rhs: Vector<N1, Der<N2>>) -> Self::Output { rhs.map(|x| self * x) }
}

impl<const N: usize, T: Scalar> MulAssign<T> for Vector<N, T> {
    fn mul_assign(&mut self, rhs: T) {
        *self = *self * rhs
    }
}

impl<const N: usize, T: Scalar> Div<T> for Vector<N, T> {
    type Output = Self;
    fn div(self, rhs: T) -> Self::Output { self.map(|x| x / rhs) }
}

impl<const N: usize, T: Scalar> Neg for Vector<N, T> {
    type Output = Self;
    fn neg(self) -> Self::Output { self.map(|x| -x) }
}

impl<const N: usize, T: Debug> Debug for Vector<N, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "(")?;
        for i in 0..N - 1 {
            write!(f, "{:?}, ", self.0[i])?;
        }
        write!(f, "{:?}", self.0[N - 1])?;
        write!(f, ")")?;
        Ok(())
    }
}

impl<const N: usize, T: Scalar> Sum for Vector<N, T> {
    fn sum<I: Iterator<Item=Self>>(iter: I) -> Self {
        iter.fold(Vector::default(), Vector::add)
    }
}

impl<T> Vector<2, T> {
    pub fn x(&self) -> T where T: Copy { self[0] }
    pub fn y(&self) -> T where T: Copy { self[1] }
    pub fn x_mut(&mut self) -> &mut T { &mut self[0] }
    pub fn y_mut(&mut self) -> &mut T { &mut self[1] }
}

impl<T> Vector<2, T> {
    pub fn new(x: T, y: T) -> Self { Vector([x, y]) }
}

impl<T> Vector<3, T> {
    pub fn x(&self) -> T where T: Copy { self[0] }
    pub fn y(&self) -> T where T: Copy { self[1] }
    pub fn z(&self) -> T where T: Copy { self[2] }
    pub fn x_mut(&mut self) -> &mut T { &mut self[0] }
    pub fn y_mut(&mut self) -> &mut T { &mut self[1] }
    pub fn z_mut(&mut self) -> &mut T { &mut self[2] }
}

impl<T> Vector<3, T> {
    pub fn new(x: T, y: T, z: T) -> Self { Vector([x, y, z]) }
    pub fn from_vec4(vec: Vec4<T>) -> Self where T: Scalar {
        Vec3::new(vec.x(), vec.y(), vec.z())
    }
    pub fn cross(self, other: Self) -> Self where T: Scalar {
        let [a1, a2, a3]: [T; 3] = self.into();
        let [b1, b2, b3]: [T; 3] = other.into();
        [a2 * b3 - a3 * b2, a3 * b1 - a1 * b3, a1 * b2 - a2 * b1].into()
    }
}

impl<T> Vector<4, T> {
    pub fn from_vec3(vec: Vec3<T>) -> Self where T: Scalar {
        Vec4::new(vec.x(), vec.y(), vec.z(), T::from(0.0))
    }
    pub fn from_position(vec: Vec3<T>) -> Self where T: Scalar {
        Vec4::new(vec.x(), vec.y(), vec.z(), T::from(1.0))
    }
    pub fn x(&self) -> T where T: Copy { self[0] }
    pub fn y(&self) -> T where T: Copy { self[1] }
    pub fn z(&self) -> T where T: Copy { self[2] }
    pub fn w(&self) -> T where T: Copy { self[3] }
    pub fn x_mut(&mut self) -> &mut T { &mut self[0] }
    pub fn y_mut(&mut self) -> &mut T { &mut self[1] }
    pub fn z_mut(&mut self) -> &mut T { &mut self[2] }
    pub fn w_mut(&mut self) -> &mut T { &mut self[3] }
}

impl<T> Vector<4, T> {
    pub fn new(x: T, y: T, z: T, w: T) -> Self { Vector([x, y, z, w]) }
}

