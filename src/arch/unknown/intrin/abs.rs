// This file is part of faster, the SIMD library for humans.
// Copyright 2017 Adam Niederer <adam.niederer@gmail.com>

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::intrin::abs::Abs;
use crate::arch::current::vecs::*;
use crate::core::mem::transmute;

impl Abs for f32x4 {
    type Out = f32x4;

    #[inline(always)]
    fn abs(&self) -> Self::Out {
        Self::Out::new(self.extract(0).abs(),
                       self.extract(1).abs(),
                       self.extract(2).abs(),
                       self.extract(3).abs())
    }
}

impl Abs for f64x2 {
    type Out = f64x2;
   
    #[inline(always)]
    fn abs(&self) -> Self::Out {
        Self::Out::new(self.extract(0).abs(),
                       self.extract(1).abs())
    }
}

impl Abs for f32x8 {
    type Out = f32x8;

    #[inline(always)]
    fn abs(&self) -> Self::Out {
        Self::Out::new(self.extract(0).abs(),
                       self.extract(1).abs(),
                       self.extract(2).abs(),
                       self.extract(3).abs(),
                       self.extract(4).abs(),
                       self.extract(5).abs(),
                       self.extract(6).abs(),
                       self.extract(7).abs())
    }
}

impl Abs for f64x4 {
    type Out = f64x4;

    #[inline(always)]
    fn abs(&self) -> Self::Out {
        Self::Out::new(self.extract(0).abs(),
                       self.extract(1).abs(),
                       self.extract(2).abs(),
                       self.extract(3).abs())
    }
}

impl Abs for i8x16 {
    type Out = u8x16;

    #[inline(always)]
    fn abs(&self) -> Self::Out {
        Self::Out::new(unsafe { transmute::<i8, u8>(self.extract(0).overflowing_abs().0) },
                       unsafe { transmute::<i8, u8>(self.extract(1).overflowing_abs().0) },
                       unsafe { transmute::<i8, u8>(self.extract(2).overflowing_abs().0) },
                       unsafe { transmute::<i8, u8>(self.extract(3).overflowing_abs().0) },
                       unsafe { transmute::<i8, u8>(self.extract(4).overflowing_abs().0) },
                       unsafe { transmute::<i8, u8>(self.extract(5).overflowing_abs().0) },
                       unsafe { transmute::<i8, u8>(self.extract(6).overflowing_abs().0) },
                       unsafe { transmute::<i8, u8>(self.extract(7).overflowing_abs().0) },
                       unsafe { transmute::<i8, u8>(self.extract(8).overflowing_abs().0) },
                       unsafe { transmute::<i8, u8>(self.extract(9).overflowing_abs().0) },
                       unsafe { transmute::<i8, u8>(self.extract(10).overflowing_abs().0) },
                       unsafe { transmute::<i8, u8>(self.extract(11).overflowing_abs().0) },
                       unsafe { transmute::<i8, u8>(self.extract(12).overflowing_abs().0) },
                       unsafe { transmute::<i8, u8>(self.extract(13).overflowing_abs().0) },
                       unsafe { transmute::<i8, u8>(self.extract(14).overflowing_abs().0) },
                       unsafe { transmute::<i8, u8>(self.extract(15).overflowing_abs().0) })
    }
}

impl Abs for i16x8 {
    type Out = u16x8;

    #[inline(always)]
    fn abs(&self) -> Self::Out {
        Self::Out::new(unsafe { transmute::<i16, u16>(self.extract(0).overflowing_abs().0) },
                       unsafe { transmute::<i16, u16>(self.extract(1).overflowing_abs().0) },
                       unsafe { transmute::<i16, u16>(self.extract(2).overflowing_abs().0) },
                       unsafe { transmute::<i16, u16>(self.extract(3).overflowing_abs().0) },
                       unsafe { transmute::<i16, u16>(self.extract(4).overflowing_abs().0) },
                       unsafe { transmute::<i16, u16>(self.extract(5).overflowing_abs().0) },
                       unsafe { transmute::<i16, u16>(self.extract(6).overflowing_abs().0) },
                       unsafe { transmute::<i16, u16>(self.extract(7).overflowing_abs().0) })
    }
}

impl Abs for i32x4 {
    type Out = u32x4;
    
    #[inline(always)]
    fn abs(&self) -> Self::Out {
        Self::Out::new(unsafe { transmute::<i32, u32>(self.extract(0).overflowing_abs().0) },
                       unsafe { transmute::<i32, u32>(self.extract(1).overflowing_abs().0) },
                       unsafe { transmute::<i32, u32>(self.extract(2).overflowing_abs().0) },
                       unsafe { transmute::<i32, u32>(self.extract(3).overflowing_abs().0) })
    }
}

impl Abs for i8x32 {
    type Out = u8x32;

    #[inline(always)]
    fn abs(&self) -> Self::Out {
        Self::Out::new(unsafe { transmute::<i8, u8>(self.extract(0).overflowing_abs().0) },
                       unsafe { transmute::<i8, u8>(self.extract(1).overflowing_abs().0) },
                       unsafe { transmute::<i8, u8>(self.extract(2).overflowing_abs().0) },
                       unsafe { transmute::<i8, u8>(self.extract(3).overflowing_abs().0) },
                       unsafe { transmute::<i8, u8>(self.extract(4).overflowing_abs().0) },
                       unsafe { transmute::<i8, u8>(self.extract(5).overflowing_abs().0) },
                       unsafe { transmute::<i8, u8>(self.extract(6).overflowing_abs().0) },
                       unsafe { transmute::<i8, u8>(self.extract(7).overflowing_abs().0) },
                       unsafe { transmute::<i8, u8>(self.extract(8).overflowing_abs().0) },
                       unsafe { transmute::<i8, u8>(self.extract(9).overflowing_abs().0) },
                       unsafe { transmute::<i8, u8>(self.extract(10).overflowing_abs().0) },
                       unsafe { transmute::<i8, u8>(self.extract(11).overflowing_abs().0) },
                       unsafe { transmute::<i8, u8>(self.extract(12).overflowing_abs().0) },
                       unsafe { transmute::<i8, u8>(self.extract(13).overflowing_abs().0) },
                       unsafe { transmute::<i8, u8>(self.extract(14).overflowing_abs().0) },
                       unsafe { transmute::<i8, u8>(self.extract(15).overflowing_abs().0) },
                       unsafe { transmute::<i8, u8>(self.extract(16).overflowing_abs().0) },
                       unsafe { transmute::<i8, u8>(self.extract(17).overflowing_abs().0) },
                       unsafe { transmute::<i8, u8>(self.extract(18).overflowing_abs().0) },
                       unsafe { transmute::<i8, u8>(self.extract(19).overflowing_abs().0) },
                       unsafe { transmute::<i8, u8>(self.extract(20).overflowing_abs().0) },
                       unsafe { transmute::<i8, u8>(self.extract(21).overflowing_abs().0) },
                       unsafe { transmute::<i8, u8>(self.extract(22).overflowing_abs().0) },
                       unsafe { transmute::<i8, u8>(self.extract(23).overflowing_abs().0) },
                       unsafe { transmute::<i8, u8>(self.extract(24).overflowing_abs().0) },
                       unsafe { transmute::<i8, u8>(self.extract(25).overflowing_abs().0) },
                       unsafe { transmute::<i8, u8>(self.extract(26).overflowing_abs().0) },
                       unsafe { transmute::<i8, u8>(self.extract(27).overflowing_abs().0) },
                       unsafe { transmute::<i8, u8>(self.extract(28).overflowing_abs().0) },
                       unsafe { transmute::<i8, u8>(self.extract(29).overflowing_abs().0) },
                       unsafe { transmute::<i8, u8>(self.extract(30).overflowing_abs().0) },
                       unsafe { transmute::<i8, u8>(self.extract(31).overflowing_abs().0) })
    }
}

impl Abs for i16x16 {
    type Out = u16x16;
    
    #[inline(always)]
    fn abs(&self) -> Self::Out {
        Self::Out::new(unsafe { transmute::<i16, u16>(self.extract(0).overflowing_abs().0) },
                       unsafe { transmute::<i16, u16>(self.extract(1).overflowing_abs().0) },
                       unsafe { transmute::<i16, u16>(self.extract(2).overflowing_abs().0) },
                       unsafe { transmute::<i16, u16>(self.extract(3).overflowing_abs().0) },
                       unsafe { transmute::<i16, u16>(self.extract(4).overflowing_abs().0) },
                       unsafe { transmute::<i16, u16>(self.extract(5).overflowing_abs().0) },
                       unsafe { transmute::<i16, u16>(self.extract(6).overflowing_abs().0) },
                       unsafe { transmute::<i16, u16>(self.extract(7).overflowing_abs().0) },
                       unsafe { transmute::<i16, u16>(self.extract(8).overflowing_abs().0) },
                       unsafe { transmute::<i16, u16>(self.extract(9).overflowing_abs().0) },
                       unsafe { transmute::<i16, u16>(self.extract(10).overflowing_abs().0) },
                       unsafe { transmute::<i16, u16>(self.extract(11).overflowing_abs().0) },
                       unsafe { transmute::<i16, u16>(self.extract(12).overflowing_abs().0) },
                       unsafe { transmute::<i16, u16>(self.extract(13).overflowing_abs().0) },
                       unsafe { transmute::<i16, u16>(self.extract(14).overflowing_abs().0) },
                       unsafe { transmute::<i16, u16>(self.extract(15).overflowing_abs().0) })
    }
}

impl Abs for i32x8 {
    type Out = u32x8;

    #[inline(always)]
    fn abs(&self) -> Self::Out {
        Self::Out::new(unsafe { transmute::<i32, u32>(self.extract(0).overflowing_abs().0) },
                       unsafe { transmute::<i32, u32>(self.extract(1).overflowing_abs().0) },
                       unsafe { transmute::<i32, u32>(self.extract(2).overflowing_abs().0) },
                       unsafe { transmute::<i32, u32>(self.extract(3).overflowing_abs().0) },
                       unsafe { transmute::<i32, u32>(self.extract(4).overflowing_abs().0) },
                       unsafe { transmute::<i32, u32>(self.extract(5).overflowing_abs().0) },
                       unsafe { transmute::<i32, u32>(self.extract(6).overflowing_abs().0) },
                       unsafe { transmute::<i32, u32>(self.extract(7).overflowing_abs().0) })
    }
}

impl Abs for i64x2 {
    type Out = u64x2;

    #[inline(always)]
    fn abs(&self) -> Self::Out {
        Self::Out::new(unsafe { transmute::<i64, u64>(self.extract(0).overflowing_abs().0) },
                       unsafe { transmute::<i64, u64>(self.extract(1).overflowing_abs().0) })
    }
}

impl Abs for i64x4 {
    type Out = u64x4;

    #[inline(always)]
    fn abs(&self) -> Self::Out {
        Self::Out::new(unsafe { transmute::<i64, u64>(self.extract(0).overflowing_abs().0) },
                       unsafe { transmute::<i64, u64>(self.extract(1).overflowing_abs().0) },
                       unsafe { transmute::<i64, u64>(self.extract(2).overflowing_abs().0) },
                       unsafe { transmute::<i64, u64>(self.extract(3).overflowing_abs().0) })
    }
}

impl Abs for i64x8 {
    type Out = u64x8;

    #[inline(always)]
    fn abs(&self) -> Self::Out {
        Self::Out::new(unsafe { transmute::<i64, u64>(self.extract(0).overflowing_abs().0) },
                       unsafe { transmute::<i64, u64>(self.extract(1).overflowing_abs().0) },
                       unsafe { transmute::<i64, u64>(self.extract(2).overflowing_abs().0) },
                       unsafe { transmute::<i64, u64>(self.extract(3).overflowing_abs().0) },
                       unsafe { transmute::<i64, u64>(self.extract(4).overflowing_abs().0) },
                       unsafe { transmute::<i64, u64>(self.extract(5).overflowing_abs().0) },
                       unsafe { transmute::<i64, u64>(self.extract(6).overflowing_abs().0) },
                       unsafe { transmute::<i64, u64>(self.extract(7).overflowing_abs().0) })
    }
}

