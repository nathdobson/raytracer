use std::iter::Sum;
use std::ops::{Add, Index, IndexMut, Mul, Sub};
use crate::math::scalar::Scalar;
use crate::math::vec::{Vec2, Vec3, Vec4, Vector};

#[derive(Copy, Clone, Default, Debug, PartialEq, PartialOrd)]
pub struct Matrix<const N: usize, T>(Vector<N, Vector<N, T>>);

pub type Mat2<T> = Matrix<2, T>;
pub type Mat3<T> = Matrix<3, T>;
pub type Mat4<T> = Matrix<4, T>;

impl<const N: usize, T> Matrix<N, T> {
    pub fn from_row_vector(rows: Vector<N, Vector<N, T>>) -> Self { Matrix(rows) }
    pub fn from_row_arrays(rows: [[T; N]; N]) -> Self {
        Matrix(Vector::<N, [T; N]>::from(rows).map(|x| x.into()))
    }
    pub fn into_row_vector(self) -> Vector<N, Vector<N, T>> { self.0 }
    pub fn into_row_array(self) -> [[T; N]; N] {
        self.0.map(|x| x.into()).into()
    }
    pub fn into_col_vector(self) -> Vector<N, Vector<N, T>> where T: Scalar {
        self.transpose().into_row_vector()
    }
    pub fn from_col_vector(v: Vector<N, Vector<N, T>>) -> Self where T: Scalar {
        Self::from_row_vector(v).transpose()
    }
    pub fn row(&self, row: usize) -> Vector<N, T> where T: Copy {
        self.0[row]
    }
    pub fn transpose(self) -> Self where T: Scalar {
        let mut result = Self::default();
        for i in 0..N {
            for j in 0..N {
                result[(i, j)] = self[(j, i)];
            }
        }
        result
    }
    pub fn from_diagonal(v: Vector<N, T>) -> Self where T: Scalar {
        let mut result = Self::default();
        for i in 0..N {
            result[(i, i)] = v[i];
        }
        result
    }
    pub fn identity() -> Self where T: Scalar {
        let mut result = Self::default();
        for i in 0..N {
            result[(i, i)] = T::from(1.0);
        }
        result
    }
    pub fn map<T2, F: FnMut(T) -> T2>(self, mut f: F) -> Matrix<N, T2> {
        Matrix(self.0.map(|x| x.map(|y| f(y))))
    }
    pub fn zip<T2>(self, other: Matrix<N, T2>) -> Matrix<N, (T, T2)> {
        Matrix(self.0.zip(other.0).map(|(x, y)| x.zip(y)))
    }
    pub fn cast<T2: From<T>>(self) -> Matrix<N, T2> {
        self.map(|x| x.into())
    }
}

impl<T> Mat2<T> {
    pub fn from_rows(x: Vec2<T>, y: Vec2<T>) -> Self { Self::from_row_vector(Vec2::new(x, y)) }
    pub fn inverse(&self) -> Self where T: Scalar {
        let [[a, b], [c, d]]: [[T; 2]; 2] = (*self).into_row_array();
        Mat2::from_row_arrays([[d, -b], [-c, a]]) * (T::from(1.0) / (a * d - b * c))
    }
}

impl<T> Mat3<T> {
    pub fn from_rows(x: Vec3<T>, y: Vec3<T>, z: Vec3<T>) -> Self { Self::from_row_vector(Vec3::new(x, y, z)) }
    pub fn from_mat4(other: &Mat4<T>) -> Self where T: Scalar {
        Self::from_rows(Vec3::from_vec4(other.row(0)),
                        Vec3::from_vec4(other.row(1)),
                        Vec3::from_vec4(other.row(2)))
    }
    pub fn inverse(&self) -> Self where T: Scalar {
        let [[a, b, c], [d, e, f], [g, h, i]] = self.into_row_array();
        Mat3::from_rows(
            Vec3::new(e * i - f * h, c * h - b * i, b * f - c * e),
            Vec3::new(f * g - d * i, a * i - c * g, c * d - a * f),
            Vec3::new(d * h - e * g, b * g - a * h, a * e - b * d),
        ) * (T::from(1.0) / (a * (e * i - f * h) - b * (d * i - f * g) + c * (d * h - e * g)))
    }
}

impl<T> Mat4<T> {
    pub fn from_rows(x: Vec4<T>, y: Vec4<T>, z: Vec4<T>, w: Vec4<T>) -> Self { Self::from_row_vector(Vec4::new(x, y, z, w)) }
    pub fn from_mat3(mat: Mat3<T>) -> Self where T: Scalar {
        Self::from_rows(
            Vec4::from_vec3(mat.row(0)),
            Vec4::from_vec3(mat.row(1)),
            Vec4::from_vec3(mat.row(2)),
            Vec4::new(T::from(0.0), T::from(0.0), T::from(0.0), T::from(1.0)),
        )
    }
    pub fn inverse(&self) -> Self where T: Scalar {
        let [
        [m00, m01, m02, m03],
        [m10, m11, m12, m13],
        [m20, m21, m22, m23],
        [m30, m31, m32, m33],
        ] = self.into_row_array();
        let a2323 = m22 * m33 - m23 * m32;
        let a1323 = m21 * m33 - m23 * m31;
        let a1223 = m21 * m32 - m22 * m31;
        let a0323 = m20 * m33 - m23 * m30;
        let a0223 = m20 * m32 - m22 * m30;
        let a0123 = m20 * m31 - m21 * m30;
        let a2313 = m12 * m33 - m13 * m32;
        let a1313 = m11 * m33 - m13 * m31;
        let a1213 = m11 * m32 - m12 * m31;
        let a2312 = m12 * m23 - m13 * m22;
        let a1312 = m11 * m23 - m13 * m21;
        let a1212 = m11 * m22 - m12 * m21;
        let a0313 = m10 * m33 - m13 * m30;
        let a0213 = m10 * m32 - m12 * m30;
        let a0312 = m10 * m23 - m13 * m20;
        let a0212 = m10 * m22 - m12 * m20;
        let a0113 = m10 * m31 - m11 * m30;
        let a0112 = m10 * m21 - m11 * m20;

        let det = T::from(1.0) / (
            m00 * (m11 * a2323 - m12 * a1323 + m13 * a1223)
                - m01 * (m10 * a2323 - m12 * a0323 + m13 * a0223)
                + m02 * (m10 * a1323 - m11 * a0323 + m13 * a0123)
                - m03 * (m10 * a1223 - m11 * a0223 + m12 * a0123)
        );

        let n00 = det * (m11 * a2323 - m12 * a1323 + m13 * a1223);
        let n01 = det * -(m01 * a2323 - m02 * a1323 + m03 * a1223);
        let n02 = det * (m01 * a2313 - m02 * a1313 + m03 * a1213);
        let n03 = det * -(m01 * a2312 - m02 * a1312 + m03 * a1212);
        let n10 = det * -(m10 * a2323 - m12 * a0323 + m13 * a0223);
        let n11 = det * (m00 * a2323 - m02 * a0323 + m03 * a0223);
        let n12 = det * -(m00 * a2313 - m02 * a0313 + m03 * a0213);
        let n13 = det * (m00 * a2312 - m02 * a0312 + m03 * a0212);
        let n20 = det * (m10 * a1323 - m11 * a0323 + m13 * a0123);
        let n21 = det * -(m00 * a1323 - m01 * a0323 + m03 * a0123);
        let n22 = det * (m00 * a1313 - m01 * a0313 + m03 * a0113);
        let n23 = det * -(m00 * a1312 - m01 * a0312 + m03 * a0112);
        let n30 = det * -(m10 * a1223 - m11 * a0223 + m12 * a0123);
        let n31 = det * (m00 * a1223 - m01 * a0223 + m02 * a0123);
        let n32 = det * -(m00 * a1213 - m01 * a0213 + m02 * a0113);
        let n33 = det * (m00 * a1212 - m01 * a0212 + m02 * a0112);
        Self::from_rows(
            Vec4::new(n00, n01, n02, n03),
            Vec4::new(n10, n11, n12, n13),
            Vec4::new(n20, n21, n22, n23),
            Vec4::new(n30, n31, n32, n33))
    }
    pub fn transform_position(&self, point: Vec3<T>) -> Vec3<T> where T: Scalar {
        Vec3::from_vec4((*self) * Vec4::from_position(point))
    }
    pub fn transform_tangent(&self, point: Vec3<T>) -> Vec3<T> where T: Scalar {
        Vec3::from_vec4((*self) * Vec4::from_vec3(point))
    }
    pub fn from_scale(scale: Vec3<T>) -> Self where T: Scalar {
        Mat4::from_mat3(Mat3::from_diagonal(scale))
    }
    pub fn from_rotation(rotation: ()) -> Self where T: Scalar { Mat4::identity() }
    pub fn from_translation(translation: Vec3<T>) -> Self where T: Scalar {
        let mut result = Mat4::identity();
        for i in 0..3 {
            result[(i, 3)] = translation[i];
        }
        result
    }
    pub fn from_scale_rotation_translation(scale: Vec3<T>, rotation: (), translation: Vec3<T>) -> Self where T: Scalar {
        let scale = Self::from_scale(scale);
        let rotation = Self::from_rotation(rotation);
        let translation = Self::from_translation(translation);
        scale * rotation * translation
    }
}

impl<const N: usize, T: Scalar> Mul<Vector<N, T>> for Matrix<N, T> {
    type Output = Vector<N, T>;
    fn mul(self, rhs: Vector<N, T>) -> Self::Output { self.0.map(|row| row.dot(rhs)) }
}

impl<const N: usize, T: Scalar> Mul<Matrix<N, T>> for Matrix<N, T> {
    type Output = Matrix<N, T>;
    fn mul(self, rhs: Matrix<N, T>) -> Self::Output {
        Matrix::from_col_vector(rhs.into_col_vector().map(|v| self * v))
    }
}

impl<const N: usize, T: Scalar> Add<Matrix<N, T>> for Matrix<N, T> {
    type Output = Matrix<N, T>;
    fn add(self, rhs: Matrix<N, T>) -> Self::Output {
        self.zip(rhs).map(|(x, y)| x + y)
    }
}

impl<const N: usize, T: Scalar> Sub<Matrix<N, T>> for Matrix<N, T> {
    type Output = Matrix<N, T>;
    fn sub(self, rhs: Matrix<N, T>) -> Self::Output {
        self.zip(rhs).map(|(x, y)| x - y)
    }
}

impl<const N: usize, T: Scalar> Mul<T> for Matrix<N, T> {
    type Output = Matrix<N, T>;
    fn mul(self, rhs: T) -> Self::Output {
        self.map(|x| x * rhs)
    }
}

impl<const N: usize, T> Index<(usize, usize)> for Matrix<N, T> {
    type Output = T;
    fn index(&self, index: (usize, usize)) -> &Self::Output { &self.0[index.0][index.1] }
}

impl<const N: usize, T> IndexMut<(usize, usize)> for Matrix<N, T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output { &mut self.0[index.0][index.1] }
}


#[test]
fn test_mat2_inverse() {
    let mat = Mat2::from_rows(Vec2::new(1.0, 2.0), Vec2::new(3.0, 4.0));
    assert_eq!(mat * mat.inverse(), Mat2::identity())
}

#[test]
fn test_mat3_inverse() {
    let mat = Mat3::from_rows(
        Vec3::new(1.0, 2.0, 3.0),
        Vec3::new(1.5, 2.5, 3.5),
        Vec3::new(1.25, 2.25, 3.75),
    );
    assert_eq!(mat * mat.inverse(), Mat3::identity())
}

#[test]
fn test_mat4_inverse() {
    let mat = Mat4::from_rows(
        Vec4::new(1.0, 2.0, 3.0, 4.0),
        Vec4::new(2.0, 4.0, 8.0, 16.0),
        Vec4::new(4.0, 16.0, 64.0, 256.0),
        Vec4::new(8.0, 64.0, 8.0, 64.0),
    );
    let diff = mat * mat.inverse() - Mat4::identity();
    for i in 0..4 {
        for j in 0..4 {
            assert!(diff[(i, j)] < 1.0e-10);
        }
    }
}