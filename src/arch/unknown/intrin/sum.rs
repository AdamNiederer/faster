use crate::arch::current::vecs::*;
use crate::vecs::*;
use crate::std::ops::Add;
use crate::intrin::upcast::Upcast;
use crate::intrin::cmp::Cmp;
use crate::intrin::abs::Abs;
use crate::intrin::sum::{Sum,UpcastSum};
use crate::intrin::transmute::Transmute;


impl_packed_sum!(u8x16, i8x16, u16x8, i16x8, u32x4, i32x4, u64x2, i64x2, f32x4, f64x2);
impl_packed_upcast_sum!(u8x16, i8x16, u16x8, i16x8, u32x4, i32x4, u64x2, i64x2, f32x4, f64x2);

mod tests {
    use crate::prelude::*;
    use crate::arch::current::vecs::*;

    macro_rules! test_packed_sum_int {
        ($vec:tt, $el:tt, $name:ident) => {
            #[test]
            fn $name() {
                // Try not to overflow
                let mut i = $el::min_value() / 64 + 1;

                while i < $el::max_value() / 64 - 1 {
                    let v = $vec::splat(i);
                    assert_eq!(v.sum(),
                               v.scalar_reduce(0 as $el, |acc, v| acc + v));
                    assert_eq!(v.sum_upcast(),
                               v.scalar_reduce(0 as i64, |acc, v| acc + (v as i64)));
                    i += $el::max_value() / 20;
                }
            }
        };
    }

    macro_rules! test_packed_sum {
        ($vec:tt, $el:tt, $name:ident) => {
            #[test]
            fn $name() {
                for i in -100..100 {
                    let v = $vec::splat(i as $el);
                    assert_eq!(v.sum(),
                               v.scalar_reduce(0 as $el, |acc, v| acc + v));
                    assert_eq!(v.sum_upcast(),
                               v.scalar_reduce(0 as i64, |acc, v| acc + (v as i64)));
                }
            }
        };
    }

    test_packed_sum_int!(u8x16, u8, test_packed_sum_u8x16);
    test_packed_sum_int!(i8x16, i8, test_packed_sum_i8x16);
    test_packed_sum_int!(u16x8, u16, test_packed_sum_u16x8);
    test_packed_sum_int!(i16x8, i16, test_packed_sum_i16x8);
    test_packed_sum_int!(u32x4, u32, test_packed_sum_u32x4);
    test_packed_sum_int!(i32x4, i32, test_packed_sum_i32x4);
    test_packed_sum_int!(u64x2, u64, test_packed_sum_u64x2);
    test_packed_sum_int!(i64x2, i64, test_packed_sum_i64x2);

    test_packed_sum!(f32x4, f32, test_packed_sum_f32x4);

    test_packed_sum!(f64x2, f64, test_packed_sum_f64x2);
}
