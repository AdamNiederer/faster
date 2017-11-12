#[macro_use]
mod macros;

mod abs;
pub use self::abs::*;
mod sqrt;
pub use self::sqrt::*;
mod transmute;
pub use self::transmute::*;
mod round;
pub use self::round::*;
mod recip;
pub use self::recip::*;
mod hadd;
pub use self::hadd::*;
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
