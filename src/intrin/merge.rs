use crate::arch::current::vecs::*;
use crate::vec_patterns::*;
use crate::intrin::transmute::*;
use crate::std::mem::transmute;

pub trait Merge {
    /// Return a vector with the first half populated by the first half of
    /// `self`, and the second half populated by the second half of `other`.
    ///
    /// ```
    /// extern crate faster;
    /// use faster::*;
    ///
    /// # fn main() {
    /// assert_eq!(u8s(2).merge_halves(u8s(3)), u8s::halfs(2, 3));
    /// # }
    /// ```
    fn merge_halves(&self, other: Self) -> Self;

    /// Return a vector containing the even elements of `self` interleaved with
    /// the odd elements of other, starting with the first element of `self`.
    ///
    /// ```
    /// extern crate faster;
    /// use faster::*;
    ///
    /// # fn main() {
    /// assert_eq!(u8s(2).merge_interleaved(u8s(3)), u8s::interleave(2, 3));
    /// # }
    /// ```
    fn merge_interleaved(&self, other: Self) -> Self;

    /// Return a vector containing the first `offset` elements of `self`, then
    /// the last `(Self::WIDTH - offset)` elements of `other`.
    ///
    /// ```
    /// extern crate faster;
    /// use faster::*;
    ///
    /// # fn main() {
    /// assert_eq!(u8s(2).merge_partitioned(u8s(3), 2), u8s::partition(2u8, 3u8, 2));
    /// # }
    /// ```
    fn merge_partitioned(&self, other: Self, offset: usize) -> Self;
}
