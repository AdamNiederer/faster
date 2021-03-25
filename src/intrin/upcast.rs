// This file is part of faster, the SIMD library for humans.
// Copyright 2017 Adam Niederer <adam.niederer@gmail.com>

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

// TODO: Upcast<i..> for u..

pub trait Upcast<T> {
    /// Return two vectors containing elements of the same value, but different
    /// type. The first vector contains the first half of `self`, and the second
    /// vector contains the second half. Both returned vectors are equal in size
    /// to `self`.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate faster;
    /// use faster::*;
    ///
    /// # fn main() {
    /// assert_eq!(i8s::halfs(2, 3).upcast(), (i16s(2), i16s(3)))
    /// # }
    /// ```
    fn upcast(self) -> (T, T);
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn upcast_i8s() {
        assert_eq!(i8s::interleave(1, 2).upcast().0, i16s::interleave(1, 2));
        assert_eq!(i8s::interleave(1, 2).upcast().1, i16s::interleave(1, 2));
    }

    #[test]
    fn upcast_u8s() {
        assert_eq!(u8s::interleave(1, 2).upcast().0, u16s::interleave(1, 2));
        assert_eq!(u8s::interleave(1, 2).upcast().1, u16s::interleave(1, 2));
    }

    #[test]
    fn upcast_i16s() {
        assert_eq!(i16s::interleave(1, 2).upcast().0, i32s::interleave(1, 2));
        assert_eq!(i16s::interleave(1, 2).upcast().1, i32s::interleave(1, 2));
    }

    #[test]
    fn upcast_u16s() {
        assert_eq!(u16s::interleave(1, 2).upcast().0, u32s::interleave(1, 2));
        assert_eq!(u16s::interleave(1, 2).upcast().1, u32s::interleave(1, 2));
    }

    #[test]
    fn upcast_i32s_i64s() {
        // TODO: Fix ugliness
        assert_eq!(
            Upcast::<i64s>::upcast(i32s::interleave(1, 2)).0,
            i64s::interleave(1, 2)
        );
        assert_eq!(
            Upcast::<i64s>::upcast(i32s::interleave(1, 2)).1,
            i64s::interleave(1, 2)
        );
    }

    #[test]
    fn upcast_i32s_f64s() {
        // TODO: Fix ugliness
        assert_eq!(
            Upcast::<f64s>::upcast(i32s::interleave(1, 2)).0,
            f64s::interleave(1.0, 2.0)
        );
        assert_eq!(
            Upcast::<f64s>::upcast(i32s::interleave(1, 2)).1,
            f64s::interleave(1.0, 2.0)
        );
    }

    #[test]
    fn upcast_u32s() {
        assert_eq!(u32s::interleave(1, 2).upcast().0, u64s::interleave(1, 2));
        assert_eq!(u32s::interleave(1, 2).upcast().1, u64s::interleave(1, 2));
    }

    #[test]
    fn upcast_f32s() {
        assert_eq!(
            f32s::interleave(1.0, 2.0).upcast(),
            (f64s::interleave(1.0, 2.0), f64s::interleave(1.0, 2.0))
        );
        assert_eq!(
            f32s::interleave(1.0, 2.0).upcast().0,
            f64s::interleave(1.0, 2.0)
        );
        assert_eq!(
            f32s::interleave(1.0, 2.0).upcast().1,
            f64s::interleave(1.0, 2.0)
        );
    }
}
