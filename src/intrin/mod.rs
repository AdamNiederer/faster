pub mod abs;
pub mod addsub;
pub mod cast;
pub mod cmp;
#[macro_use] pub mod destride;
pub mod downcast;
#[macro_use] pub mod endian;
#[macro_use] pub mod eq;
pub mod hadd;
pub mod hsub;
#[macro_use] pub mod macros;
#[macro_use] pub mod merge;
#[macro_use] pub mod popcnt;
pub mod recip;
pub mod round;
pub mod rsqrt;
#[macro_use] pub mod sum;
pub mod saturating_add;
pub mod saturating_hadd;
pub mod saturating_hsub;
pub mod saturating_sub;
pub mod sqrt;
#[macro_use] pub mod transmute;
pub mod upcast;


// We use an internal prelude not to clutter the namespace when we import
// from actual prelude. 
pub(crate) mod prelude {
    pub use super::abs::*;
    pub use super::addsub::*;
    pub use super::cast::*;
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
    pub use super::sqrt::*;
    pub use super::transmute::*;
    pub use super::upcast::*;
}