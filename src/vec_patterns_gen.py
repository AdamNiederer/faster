# This file is part of faster, the SIMD library for humans.
# Copyright 2017 Adam Niederer <adam.niederer@gmail.com>

# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http:#mozilla.org/MPL/2.0/.

# Who needs procedural macros when you have code generators?

vecs = ["u8x64", "u8x32", "u8x16", "i8x64", "i8x32", "i8x16", "u16x32", "u16x16", "u16x8", "i16x32", "i16x16", "i16x8", "u32x16", "u32x8", "u32x4", "i32x16", "i32x8", "i32x4", "f32x16", "f32x8", "f32x4", "u64x8", "u64x4", "u64x2", "i64x8", "i64x4", "i64x2", "f64x8", "f64x4", "f64x2"]
lens = [int(v.split("x")[1]) for v in vecs]
els = [v.split("x")[0] for v in vecs]
elsz = [int(el[1:]) for el in els]
feats = [{512: "avx512-notyet", 256: "avx2", 128: "sse4.1"}[l * e]
         for l, e in zip(lens, elsz)]
blends = [{512: "_mm512_mask_mov_epi8", 256: "_mm256_blendv_epi8", 128: "_mm_blendv_epi8"}[l * e]
          for l, e in zip(lens, elsz)]
masks = ["u" + el[1:] for el in els]

print("""// This file is part of faster, the SIMD library for humans.
// Copyright 2017 Adam Niederer <adam.niederer@gmail.com>

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

// This file is machine-generated. See vec_patterns_gen.py for more info.""")

print("""
use vecs::*;
use core_or_std::mem::transmute;
use stdsimd::vendor::*;

/// Constructors which may be used to instantiate vectors with patterned data.
pub trait PackedPattern : Packed {
    /// Return a vector whose first `Self::WIDTH / 2` elements are `hi`, and
    /// whose last `Self::WIDTH / 2` elements are `lo`.
    fn halfs(hi: Self::Scalar, lo: Self::Scalar) -> Self;

    /// Return a vector containing `hi` at every even index, and lo at every odd
    /// index.
    fn interleave(hi: Self::Scalar, lo: Self::Scalar) -> Self;

    /// Return a vector whose first `off` elements are `hi`, and whose last
    /// `Self::WIDTH - off` elements are `lo`.
    fn partition(hi: Self::Scalar, lo: Self::Scalar, off: usize) -> Self;

    /// Return a vector whose first `off` elements are memset to 0x00, and whose
    /// last `Self::WIDTH - off` elements are memset to 0xFF.
    fn partition_mask(off: usize) -> Self;

    /// Return a vector made entirely of ones.
    fn ones() -> Self;

    /// Return a vector made entirely of zeroes.
    fn zeroes() -> Self;
}

const PART_MASK: [u8; 128] = [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                              0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                              0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                              0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                              0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                              0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                              0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                              0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                              0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                              0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                              0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                              0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                              0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                              0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                              0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                              0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF];
""")

for e, v, l, f, b, s, m in zip(els, vecs, lens, feats, blends, elsz, masks):
    # Generate halfs
    print(f"impl PackedPattern for {v} {{")
    print(f"    #[inline(always)]")
    print(f"    fn halfs(hi: Self::Scalar, lo: Self::Scalar) -> Self {{")
    first = ", ".join("hi" for _ in range(l // 2))
    second = ", ".join("lo" for _ in range(l // 2))
    print(f"        Self::new({first}, {second})")
    print(f"    }}\n")

    # Generate interleave
    print(f"    #[inline(always)]")
    print(f"    fn interleave(hi: Self::Scalar, lo: Self::Scalar) -> Self {{")
    args = ", ".join("hi, lo" for _ in range(l // 2))
    print(f"        Self::new({args})")
    print(f"    }}")

    # Generate partition_mask
    print(f"""
    #[inline(always)]
    fn partition_mask(off: usize) -> Self {{
        debug_assert!(off <= Self::WIDTH);
        debug_assert!(off * Self::Scalar::SIZE <= 64);
        Self::load(unsafe {{ transmute(&PART_MASK[..]) }}, 64 / Self::Scalar::SIZE - off)
    }}""",)

    # Generate partition polyfill
    print(f"""
    #[inline(always)]
    #[cfg(target_feature = "{f}")]
    fn partition(hi: Self::Scalar, lo: Self::Scalar, off: usize) -> Self {{
        unsafe {{ transmute({b}(transmute(Self::splat(hi)), transmute(Self::splat(lo)), transmute(Self::partition_mask(off)))) }}
    }}
    """)

    # Generate partition polyfill
    print(f"    #[inline(always)]")
    print(f"    #[cfg(not(target_feature = \"{f}\"))]")
    print(f"    fn partition(hi: Self::Scalar, lo: Self::Scalar, off: usize) -> Self {{")
    print(f"        assert!(off <= Self::WIDTH);")
    print(f"        match off {{")
    for i in range(0, l + 1):
        first = ", ".join("hi" for _ in range(i))
        second = ", ".join("lo" for _ in range(l - i))
        args = ", ".join((first, second)).strip(", ")
        print(f"            {i} => Self::new({args}),")
    print(f"            _ => unreachable!()")
    print(f"        }}")
    print(f"    }}")

    # Generate ones & zeroes
    print(f"""
    /// Return a vector made entirely of ones.
    fn ones() -> Self {{
        Self::splat(unsafe {{ transmute(0x{'F' * (s // 4)}{m}) }})
    }}

    /// Return a vector made entirely of zeroes.
    fn zeroes() -> Self {{
        Self::splat(unsafe {{ transmute(0x{'0' * (s // 4)}{m}) }})
    }}""")


    print(f"}}\n")
