use std::cmp::Ordering;
use crate::math::scalar::Scalar;

#[derive(Debug)]
pub struct ScalarKey<A, B>(A, B);

impl<A, B> Eq for ScalarKey<A, B> where A: Scalar {}

impl<A, B> PartialEq<Self> for ScalarKey<A, B> where A: Scalar {
    fn eq(&self, other: &Self) -> bool {
        self.0.real_eq(other.0)
    }
}

impl<A, B> PartialOrd<Self> for ScalarKey<A, B> where A: Scalar {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.0.real_cmp(other.0))
    }
}

impl<A, B> Ord for ScalarKey<A, B> where A: Scalar {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.real_cmp(other.0)
    }
}

impl<A: Scalar, B> ScalarKey<A, B> {
    pub fn new(x: A, y: B) -> Self {
        assert!(x.not_nan());
        ScalarKey(x, y)
    }
    pub fn key(&self) -> &A { &self.0 }
    pub fn value(&self) -> &B { &self.1 }
}