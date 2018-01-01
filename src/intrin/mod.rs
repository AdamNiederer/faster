// This file is part of faster, the SIMD library for humans.
// Copyright 2017 Adam Niederer <adam.niederer@gmail.com>

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![allow(unused_imports)]

#[macro_use]
mod macros;

mod abs;
pub use self::abs::*;
mod sqrt;
pub use self::sqrt::*;
mod transmute;
pub use self::transmute::*;
mod cast;
pub use self::cast::*;
mod upcast;
pub use self::upcast::*;
mod downcast;
pub use self::downcast::*;
mod round;
pub use self::round::*;
mod recip;
pub use self::recip::*;
mod hadd;
pub use self::hadd::*;
mod hsub;
pub use self::hsub::*;
mod rsqrt;
pub use self::rsqrt::*;
mod cmp;
pub use self::cmp::*;
mod saturating_add;
pub use self::saturating_add::*;
mod saturating_hadd;
pub use self::saturating_hadd::*;
mod saturating_sub;
pub use self::saturating_sub::*;
mod saturating_hsub;
pub use self::saturating_hsub::*;
mod addsub;
pub use self::addsub::*;
mod merge;
pub use self::merge::*;
mod swizzle;
pub use self::swizzle::*;
