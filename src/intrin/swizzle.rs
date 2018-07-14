pub trait Swizzle {
    /// Return a vector containing elements of self, but with even and odd
    /// elements swapped in-place. For (n = 0, 2, ... Self::WIDTH), elements at
    /// indices n and n + 1 are swapped.
    ///
    /// ```
    /// extern crate faster;
    /// use faster::*;
    ///
    /// # fn main() {
    /// assert_eq!(u8s::interleave(2, 1).flip(), u8s::interleave(1, 2));
    /// assert_eq!(u64s::interleave(2, 1).flip(), u64s::interleave(1, 2));
    /// # }
    /// ```
    fn flip(&self) -> Self;
}
