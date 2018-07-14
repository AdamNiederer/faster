// This file is part of faster, the SIMD library for humans.
// Copyright 2017 Adam Niederer <adam.niederer@gmail.com>

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![allow(unused_imports)]

mod abs;
pub use self::abs::*;
mod cmp;
pub use self::cmp::*;
mod destride;
pub use self::destride::*;
mod downcast;
pub use self::downcast::*;
mod endian;
pub use self::endian::*;
mod eq;
pub use self::eq::*;
mod hadd;
pub use self::hadd::*;
mod hsub;
pub use self::hsub::*;
mod merge;
pub use self::merge::*;
mod popcnt;
pub use self::popcnt::*;
mod recip;
pub use self::recip::*;
mod round;
pub use self::round::*;
mod sum;
pub use self::rsqrt::*;
mod rsqrt;
pub use self::sum::*;
mod saturating_add;
pub use self::saturating_add::*;
mod saturating_hadd;
pub use self::saturating_hadd::*;
mod saturating_sub;
pub use self::saturating_sub::*;
mod saturating_hsub;
pub use self::saturating_hsub::*;
mod sqrt;
pub use self::sqrt::*;
mod transmute;
pub use self::transmute::*;
mod upcast;
pub use self::upcast::*;


//
//mod abs;
//pub use self::abs::*;
//mod sqrt;
//pub use self::sqrt::*;
//mod transmute;
//pub use self::transmute::*;
//mod cast;
//pub use self::cast::*;
//mod upcast;
//pub use self::upcast::*;
//mod downcast;
//pub use self::downcast::*;
//mod round;
//pub use self::round::*;
//mod recip;
//pub use self::recip::*;
//mod hadd;
//pub use self::hadd::*;
//mod rsqrt;
//pub use self::rsqrt::*;
//mod cmp;
//pub use self::cmp::*;
//mod eq;
//pub use self::eq::*;
//mod saturating_add;
//pub use self::saturating_add::*;
//mod saturating_hadd;
//pub use self::saturating_hadd::*;
//mod saturating_sub;
//pub use self::saturating_sub::*;
//mod saturating_hsub;
//pub use self::saturating_hsub::*;
//mod addsub;
//pub use self::addsub::*;
//mod merge;
//pub use self::merge::*;
//mod swizzle;
//pub use self::swizzle::*;
//mod endian;
//pub use self::endian::*;
//mod sum;
//pub use self::sum::*;
//mod popcnt;
//pub use self::popcnt::*;
//mod destride;
//pub use self::destride::*;
