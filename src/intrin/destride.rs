use crate::intrin::merge::*;
use crate::intrin::transmute::*;
use crate::std::mem::transmute;


pub trait Destride : Sized {
    fn destride_two(self, other: Self) -> (Self, Self);
    fn destride_four(self, b: Self, c: Self, d: Self) -> (Self, Self, Self, Self);
}
