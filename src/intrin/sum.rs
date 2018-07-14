use crate::std::ops::Add;
use crate::vecs::*;
use crate::intrin::upcast::Upcast;
use crate::intrin::cmp::Cmp;
use crate::intrin::abs::Abs;
use crate::intrin::transmute::Transmute;

pub trait Sum : Packed {
    /// Return a scalar equivalent to the sum of all elements of this vector.
    fn sum(&self) -> Self::Scalar;
}

pub trait UpcastSum :  {
    /// Return a scalar equivalent to the sum of all elements of this vector,
    /// but collect the result in an i64 rather than the vector's type.
    fn sum_upcast(&self) -> i64;
}

