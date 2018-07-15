// This file is part of faster, the SIMD library for humans.
// Copyright 2017 Adam Niederer <adam.niederer@gmail.com>

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub trait Merge {
    /// Return a vector with the first half populated by the first half of
    /// `self`, and the second half populated by the second half of `other`.
    ///
    /// ```
    /// extern crate faster;
    /// use faster::*;
    ///
    /// # fn main() {
    /// assert_eq!(u8s(2).merge_halves(u8s(3)), u8s::halfs(2, 3));
    /// # }
    /// ```
    fn merge_halves(&self, other: Self) -> Self;

    /// Return a vector containing the even elements of `self` interleaved with
    /// the odd elements of other, starting with the first element of `self`.
    ///
    /// ```
    /// extern crate faster;
    /// use faster::*;
    ///
    /// # fn main() {
    /// assert_eq!(u8s(2).merge_interleaved(u8s(3)), u8s::interleave(2, 3));
    /// # }
    /// ```
    fn merge_interleaved(&self, other: Self) -> Self;

    /// Return a vector containing the first `offset` elements of `self`, then
    /// the last `(Self::WIDTH - offset)` elements of `other`.
    ///
    /// ```
    /// extern crate faster;
    /// use faster::*;
    ///
    /// # fn main() {
    /// assert_eq!(u8s(2).merge_partitioned(u8s(3), 2), u8s::partition(2u8, 3u8, 2));
    /// # }
    /// ```
    fn merge_partitioned(&self, other: Self, offset: usize) -> Self;
}

macro_rules! impl_packed_merge {
    ($vec:ty, $uvec:tt, $uscl:tt, $mmfn:expr, $feat:expr, ($($a:expr),*), ($($b:expr),*), $($na:expr, $nb:expr),*) => {
        #[cfg(not(target_feature = $feat))]
        impl Merge for $vec {

            #[inline(always)]
            fn merge_halves(&self, other: Self) -> Self {
                unsafe {
                    Self::new($(self.extract_unchecked($a)),*,
                              $(other.extract_unchecked($b)),*)
                }
            }

            #[inline(always)]
            fn merge_interleaved(&self, other: Self) -> Self {
                unsafe {
                    Self::new($(self.extract_unchecked($na), other.extract_unchecked($nb)),*)
                }
            }

            #[inline(always)]
            fn merge_partitioned(&self, other: Self, offset: usize) -> Self {
                assert!(offset < Self::WIDTH);
                let mut ret = self.clone();
                for i in offset..Self::WIDTH {
                    unsafe {
                        ret = ret.replace_unchecked(i, other.extract_unchecked(i));
                    }
                }
                ret
            }
        }

        #[cfg(target_feature = $feat)]
        impl Merge for $vec {

            #[inline(always)]
            fn merge_halves(&self, other: Self) -> Self {
                unsafe {
                    transmute($mmfn(
                        self.be_i8s(), other.be_i8s(),
                        transmute($uvec::halfs($uscl::min_value(), $uscl::max_value()))))
                }
            }

            #[inline(always)]
            fn merge_interleaved(&self, other: Self) -> Self {
                unsafe {
                    transmute($mmfn(
                        self.be_i8s(), other.be_i8s(),
                        transmute($uvec::interleave($uscl::min_value(), $uscl::max_value()))))
                }
            }

            #[inline(always)]
            fn merge_partitioned(&self, other: Self, offset: usize) -> Self {
                unsafe {
                    transmute($mmfn(
                        self.be_i8s(), other.be_i8s(),
                        transmute(Self::partition_mask(offset))))
                }
            }
        }
    }
}

macro_rules! test_packed_merge {
    (($($vec:tt),*), ($($fn:ident),*)) => {
        $(
            #[test]
            fn $fn() {
                let asc = 30i32 as <$vec as Packed>::Scalar;
                let bsc = 5i32 as <$vec as Packed>::Scalar;
                let a = $vec::splat(asc);
                let b = $vec::splat(bsc);
                assert_eq!(a.merge_interleaved(b), $vec::interleave(asc, bsc));
                assert_eq!(b.merge_interleaved(a), $vec::interleave(bsc, asc));

                assert_eq!(a.merge_halves(b), $vec::halfs(asc, bsc));
                assert_eq!(b.merge_halves(a), $vec::halfs(bsc, asc));

                for i in 0..$vec::WIDTH {
                    assert_eq!(a.merge_partitioned(b, i), $vec::partition(asc, bsc, i));
                    assert_eq!(b.merge_partitioned(a, i), $vec::partition(bsc, asc, i));
                }
            }
        )*
    }
}
