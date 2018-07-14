#[cfg(target_feature = "sse")] pub mod x86;
#[cfg(not(target_feature = "sse"))] pub mod shim;

#[cfg(target_feature = "sse")] pub use self::x86 as current;
#[cfg(not(target_feature = "sse"))] pub use self::shim as current;


/*
pub mod current {

    #[cfg(target_feature = "sse")]
    pub use super::x86::*;

    #[cfg(not(target_feature = "sse"))]
    pub use super::shim::*;
}
*/

//pub use crate::shimvecs::{u8x64, u8x32, u8x16, i8x64, i8x32, i8x16, u16x32, u16x16, u16x8, i16x32, i16x16, i16x8, u32x16, u32x8, u32x4, i32x16, i32x8, i32x4, f32x16, f32x8, f32x4, u64x8, u64x4, u64x2, i64x8, i64x4, i64x2, f64x8, f64x4, f64x2};
