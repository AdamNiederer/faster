// Macros contains all "common code" we need to use either native intrinsics
// or common fallbacks. 
#[macro_use] 
pub mod common;

//#[cfg(target_feature = "sse")] pub mod x86;
//#[cfg(not(target_feature = "sse"))] 
pub mod unknown;


//#[cfg(target_feature = "sse")] pub use self::x86 as current;
//#[cfg(not(target_feature = "sse"))] 
pub use self::unknown as current;
