// This file is part of faster, the SIMD library for humans.
// Copyright 2017 Adam Niederer <adam.niederer@gmail.com>

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![allow(unused_imports)]

mod abs;
mod addsub;
mod cmp;
mod destride;
mod downcast;
mod endian;
mod eq;
mod hadd;
mod hsub;
mod merge;
mod popcnt;
mod recip;
mod round;
mod sum;
mod rsqrt;
mod saturating_add;
mod saturating_hadd;
mod saturating_sub;
mod saturating_hsub;
mod sqrt;
mod transmute;
mod upcast;


// We use an internal prelude not to clutter the namespace when we import
// from actual prelude. 
pub mod prelude {
    pub use super::abs::*;
    pub use super::addsub::*;
    pub use super::cmp::*;
    pub use super::destride::*;
    pub use super::downcast::*;
    pub use super::endian::*;
    pub use super::eq::*;
    pub use super::hadd::*;
    pub use super::hsub::*;
    pub use super::merge::*;
    pub use super::popcnt::*;
    pub use super::recip::*;
    pub use super::round::*;
    pub use super::rsqrt::*;
    pub use super::sum::*;
    pub use super::saturating_add::*;
    pub use super::saturating_hadd::*;
    pub use super::saturating_hsub::*;
    pub use super::saturating_sub::*;
    pub use super::transmute::*;
    pub use super::upcast::*;
}
