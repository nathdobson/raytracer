use std::iter::Sum;
use std::mem;
use std::ops::Div;
use ordered_float::NotNan;
use crate::math::scalar::Scalar;

pub trait Itertools2: Iterator {
    fn average<T: Sum<Self::Item> + Div<f64>>(self) -> T::Output where Self: Sized {
        struct Wrapper<'a, I> {
            count: &'a mut usize,
            inner: I,
        }
        impl<'a, I: Iterator> Iterator for Wrapper<'a, I> {
            type Item = I::Item;
            fn next(&mut self) -> Option<Self::Item> {
                let result = self.inner.next();
                if result.is_some() {
                    *self.count += 1;
                }
                result
            }
        }
        let mut count = 0;
        let sum = T::sum(Wrapper { count: &mut count, inner: self });
        sum / (count as f64)
    }
    fn peeker(self) -> Peeker<Self> where Self: Sized { Peeker::new(self) }
    fn minimum(self) -> Option<Self::Item> where Self::Item: Scalar, Self: Sized {
        self.min_by(|x, y| x.real_cmp(*y))
    }
}

impl<T> Itertools2 for T where T: Iterator {}

#[derive(Debug)]
pub struct Peeker<I: Iterator> {
    next: Option<I::Item>,
    iter: I,
}

impl<I: Iterator> Peeker<I> {
    pub fn new(mut iter: I) -> Self {
        Peeker { next: iter.next(), iter }
    }
    pub fn peek(&self) -> Option<&I::Item> {
        self.next.as_ref()
    }
}

impl<I: Iterator> Iterator for Peeker<I> {
    type Item = I::Item;
    fn next(&mut self) -> Option<Self::Item> {
        mem::replace(&mut self.next, self.iter.next())
    }
}