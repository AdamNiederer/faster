use crate::arch::current::vecs::*;
use crate::intrin::destride::*;


impl Destride for u8x16 {
    #[inline(always)]
    fn destride_two(self, other: Self) -> (Self, Self) {
        destride_two_polyfill!(self, other, 0, 2, 4, 6, 8, 10, 12, 14)
    }

    #[inline(always)]
    fn destride_four(self, b: Self, c: Self, d: Self) -> (Self, Self, Self, Self) {
        destride_four_polyfill!(self, b, c, d, 0, 4, 8, 12)
    }
}

impl Destride for u8x32 {
    #[inline(always)]
    fn destride_two(self, other: Self) -> (Self, Self) {
        destride_two_polyfill!(self, other, 0, 2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 22, 24, 26, 28, 30)
    }

    #[inline(always)]
    fn destride_four(self, b: Self, c: Self, d: Self) -> (Self, Self, Self, Self) {
        destride_four_polyfill!(self, b, c, d, 0, 4, 8, 12, 16, 20, 24, 28)
    }
}

impl Destride for i8x16 {
    #[inline(always)]
    fn destride_two(self, other: Self) -> (Self, Self) {
        destride_two_polyfill!(self, other, 0, 2, 4, 6, 8, 10, 12, 14)
    }

    #[inline(always)]
    fn destride_four(self, b: Self, c: Self, d: Self) -> (Self, Self, Self, Self) {
        destride_four_polyfill!(self, b, c, d, 0, 4, 8, 12)
    }
}

impl Destride for i8x32 {
    #[inline(always)]
    fn destride_two(self, other: Self) -> (Self, Self) {
        destride_two_polyfill!(self, other, 0, 2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 22, 24, 26, 28, 30)
    }

    #[inline(always)]
    fn destride_four(self, b: Self, c: Self, d: Self) -> (Self, Self, Self, Self) {
        destride_four_polyfill!(self, b, c, d, 0, 4, 8, 12, 16, 20, 24, 28)
    }
}

macro_rules! impl_destride {
    ($t:ty, $($two:expr, $four:expr),*) => {
        impl Destride for $t {
            #[inline(always)]
            fn destride_two(self, other: Self) -> (Self, Self) {
                destride_two_polyfill!(self, other, $($two, $four),*)
            }

            #[inline(always)]
            fn destride_four(self, b: Self, c: Self, d: Self) -> (Self, Self, Self, Self) {
                destride_four_polyfill!(self, b, c, d, $($two),*)
            }
        }
    }
}

impl_destride!(u16x16, 0, 2, 4, 6, 8, 10, 12, 14);
impl_destride!(u16x8, 0, 2, 4, 6);
impl_destride!(i16x16, 0, 2, 4, 6, 8, 10, 12, 14);
impl_destride!(i16x8, 0, 2, 4, 6);

impl_destride!(u32x8, 0, 2, 4, 6);
impl_destride!(u32x4, 0, 2);
impl_destride!(i32x8, 0, 2, 4, 6);
impl_destride!(i32x4, 0, 2);
