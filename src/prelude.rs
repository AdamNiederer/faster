// This file is part of faster, the SIMD library for humans.
// Copyright 2017 Adam Niederer <adam.niederer@gmail.com>

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub use crate::arch::current::intrin::prelude::*;
pub use crate::arch::current::vecs::{f32s, f64s, i16s, i32s, i64s, i8s, u16s, u32s, u64s, u8s};
pub use crate::into_iters::*;
pub use crate::intrin::prelude::*;
pub use crate::iters::*;
pub use crate::stride::*;
pub use crate::stride_zip::*;
pub use crate::vecs::{Packed, Pattern};
pub use crate::zip::*;
