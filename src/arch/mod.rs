
//#[cfg(target_feature = "sse")]
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub mod x86;

#[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))] 
pub mod unknown;


#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub use self::x86 as current;

#[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
pub use self::unknown as current;
