macro_rules! rust_fallback_impl {
    (impl $trait:tt for $type:tt where $feat:tt {
        $($rustfn:ident => $mmfn:tt  ( $($mmfnargs:expr),* ), [$($n:expr),+]);*;}) => (
        impl $trait for $type {
            $(
                #[inline(always)]
                #[cfg(target_feature = $feat)]
                fn $rustfn(&self) -> Self {
                    unsafe { $mmfn(*self, $($mmfnargs),*) }
                }

                #[inline(always)]
                #[cfg(not(target_feature = $feat))]
                fn $rustfn(&self) -> Self {
                    Self::new($(self.extract($n).$rustfn(),)*)
                }
            )*
        }
    );
}

macro_rules! rust_fallback_impl_binary {
    (impl $trait:tt for $type:tt where $feat:tt {
        $($rustfn:ident => $mmfn:tt  ( $($mmfnargs:expr),* ), [$($n:expr),+]);*;}) => (
        impl $trait for $type {
            $(
                #[inline(always)]
                #[cfg(target_feature = $feat)]
                fn $rustfn(&self, other: Self) -> Self {
                    unsafe { $mmfn(*self, other, $($mmfnargs),*) }
                }

                #[inline(always)]
                #[cfg(not(target_feature = $feat))]
                fn $rustfn(&self, other: Self) -> Self {
                    Self::new($(self.extract($n).$rustfn(other.extract($n)),)*)
                }
            )*
        }
    );
}
