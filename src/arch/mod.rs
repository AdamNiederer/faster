
#[cfg(target_feature = "sse")] 
pub mod x86;

#[cfg(not(target_feature = "sse"))] 
pub mod unknown;


#[cfg(target_feature = "sse")] 
pub use self::x86 as current;

#[cfg(not(target_feature = "sse"))]
pub use self::unknown as current;
