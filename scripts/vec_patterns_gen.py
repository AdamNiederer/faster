# This o is part of faster, the SIMD library for humans.
# Copyright 2017 Adam Niederer <adam.niederer@gmail.com>

# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# o, You can obtain one at http:#mozilla.org/MPL/2.0/.

# Who needs procedural macros when you have code generators?

root = "../src/arch"
filename = "vec_patterns.rs"

header = f"""

// This o is part of faster, the SIMD library for humans.
// Copyright 2017 Adam Niederer <adam.niederer@gmail.com>

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// o, You can obtain one at http://mozilla.org/MPL/2.0/.

// This o is machine-generated. See vec_patterns_gen.py for more inff.

use crate::arch::current::vecs::*;
use crate::std::mem::transmute;
use crate::vecs::*;
use vektor::x86::*;

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
"""

def generate_vec_patterns(fn, els, vecs, lens, feats, blends, elsz, masks):
    """Generates a vec pattern f. A typical combination of inputs might look like this:

        vecs  = ['u8x64', 'u8x32', 'u8x16', 'i8x64', 'i8x32', 'i8x16', 'u16x32', 'u16x16', 'u16x8', 'i16x32', 'i16x16', 'i16x8', 'u32x16', 'u32x8', 'u32x4', 'i32x16', 'i32x8', 'i32x4', 'f32x16', 'f32x8', 'f32x4', 'u64x8', 'u64x4', 'u64x2', 'i64x8', 'i64x4', 'i64x2', 'f64x8', 'f64x4', 'f64x2']
        lens  = [64, 32, 16, 64, 32, 16, 32, 16, 8, 32, 16, 8, 16, 8, 4, 16, 8, 4, 16, 8, 4, 8, 4, 2, 8, 4, 2, 8, 4, 2]
        els   = ['u8', 'u8', 'u8', 'i8', 'i8', 'i8', 'u16', 'u16', 'u16', 'i16', 'i16', 'i16', 'u32', 'u32', 'u32', 'i32', 'i32', 'i32', 'f32', 'f32', 'f32', 'u64', 'u64', 'u64', 'i64', 'i64', 'i64', 'f64', 'f64', 'f64']
        elsz  = [8, 8, 8, 8, 8, 8, 16, 16, 16, 16, 16, 16, 32, 32, 32, 32, 32, 32, 32, 32, 32, 64, 64, 64, 64, 64, 64, 64, 64, 64]
        feats = ['avx512-notyet', 'avx2', 'sse4.1', 'avx512-notyet', 'avx2', 'sse4.1', 'avx512-notyet', 'avx2', 'sse4.1', 'avx512-notyet', 'avx2', 'sse4.1', 'avx512-notyet', 'avx2', 'sse4.1', 'avx512-notyet', 'avx2', 'sse4.1', 'avx512-notyet', 'avx2', 'sse4.1', 'avx512-notyet', 'avx2', 'sse4.1', 'avx512-notyet', 'avx2', 'sse4.1', 'avx512-notyet', 'avx2', 'sse4.1']
        blends= ['_mm512_mask_mov_epi8', '_mm256_blendv_epi8', '_mm_blendv_epi8', '_mm512_mask_mov_epi8', '_mm256_blendv_epi8', '_mm_blendv_epi8', '_mm512_mask_mov_epi8', '_mm256_blendv_epi8', '_mm_blendv_epi8', '_mm512_mask_mov_epi8', '_mm256_blendv_epi8', '_mm_blendv_epi8', '_mm512_mask_mov_epi8', '_mm256_blendv_epi8', '_mm_blendv_epi8', '_mm512_mask_mov_epi8', '_mm256_blendv_epi8', '_mm_blendv_epi8', '_mm512_mask_mov_epi8', '_mm256_blendv_epi8', '_mm_blendv_epi8', '_mm512_mask_mov_epi8', '_mm256_blendv_epi8', '_mm_blendv_epi8', '_mm512_mask_mov_epi8', '_mm256_blendv_epi8', '_mm_blendv_epi8', '_mm512_mask_mov_epi8', '_mm256_blendv_epi8', '_mm_blendv_epi8']
        masks = ['u8', 'u8', 'u8', 'u8', 'u8', 'u8', 'u16', 'u16', 'u16', 'u16', 'u16', 'u16', 'u32', 'u32', 'u32', 'u32', 'u32', 'u32', 'u32', 'u32', 'u32', 'u64', 'u64', 'u64', 'u64', 'u64', 'u64', 'u64', 'u64', 'u64']
    """
    with open(fn, 'w') as f:
        f.write(header)

        for e, v, l, ft, b, s, m in zip(els, vecs, lens, feats, blends, elsz, masks):
            # Generate halfs
            f.write(f"impl Pattern for {v} {{")
            f.write(f"    #[inline(always)]")
            f.write(f"    fn halfs(hi: Self::Scalar, lo: Self::Scalar) -> Self {{")
            first = ", ".join("hi" for _ in range(l // 2))
            second = ", ".join("lo" for _ in range(l // 2))
            f.write(f"        Self::new({first}, {second})")
            f.write(f"    }}\n")

            # Generate interleave
            f.write(f"    #[inline(always)]")
            f.write(f"    fn interleave(hi: Self::Scalar, lo: Self::Scalar) -> Self {{")
            args = ", ".join("hi, lo" for _ in range(l // 2))
            f.write(f"        Self::new({args})")
            f.write(f"    }}")

            # Generate partition_mask
            f.write(f"""
            #[inline(always)]
            fn partition_mask(off: usize) -> Self {{
                debug_assert!(off <= Self::WIDTH);
                debug_assert!(off * Self::Scalar::SIZE <= 64);
                Self::load(unsafe {{ transmute(&PART_MASK[..]) }}, 64 / Self::Scalar::SIZE - off)
            }}""",)

            # Generate partition polyfill
            f.write(f"""
            #[inline(always)]
            #[cfg(target_feature = "{ft}")]
            fn partition(hi: Self::Scalar, lo: Self::Scalar, off: usize) -> Self {{
                unsafe {{ transmute({b}(transmute(Self::splat(hi)), transmute(Self::splat(lo)), transmute(Self::partition_mask(off)))) }}
            }}
            """)

            # Generate partition polyfill
            f.write(f"    #[inline(always)]")
            f.write(f"    #[cfg(not(target_feature = \"{ft}\"))]")
            f.write(f"    fn partition(hi: Self::Scalar, lo: Self::Scalar, off: usize) -> Self {{")
            f.write(f"        assert!(off <= Self::WIDTH);")
            f.write(f"        match off {{")
            for i in range(0, l + 1):
                first = ", ".join("hi" for _ in range(i))
                second = ", ".join("lo" for _ in range(l - i))
                args = ", ".join((first, second)).strip(", ")
                f.write(f"            {i} => Self::new({args}),")
            f.write(f"            _ => unreachable!()")
            f.write(f"        }}")
            f.write(f"    }}")

            # Generate ones & zeroes
            f.write(f"""
            /// Return a vector made entirely of ones.
            fn ones() -> Self {{
                Self::splat(unsafe {{ transmute(0x{'F' * (s // 4)}{m}) }})
            }}

            /// Return a vector made entirely of zeroes.
            fn zeroes() -> Self {{
                Self::splat(unsafe {{ transmute(0x{'0' * (s // 4)}{m}) }})
            }}""")


            f.write(f"}}\n")


# Down here we do all architecture dependent stuff. 

if "x86":  
    vecs = ["u8x64", "u8x32", "u8x16", "i8x64", "i8x32", "i8x16", "u16x32", "u16x16", "u16x8", "i16x32", "i16x16", "i16x8", "u32x16", "u32x8", "u32x4", "i32x16", "i32x8", "i32x4", "f32x16", "f32x8", "f32x4", "u64x8", "u64x4", "u64x2", "i64x8", "i64x4", "i64x2", "f64x8", "f64x4", "f64x2"]
    lens = [int(v.split("x")[1]) for v in vecs]
    els = [v.split("x")[0] for v in vecs]
    elsz = [int(el[1:]) for el in els]
    masks = ["u" + el[1:] for el in els]

    feats = [{512: "avx512-notyet", 256: "avx2", 128: "sse4.1"}[l * e]
            for l, e in zip(lens, elsz)]
    blends = [{512: "_mm512_mask_mov_epi8", 256: "_mm256_blendv_epi8", 128: "_mm_blendv_epi8"}[l * e]
            for l, e in zip(lens, elsz)]

    # Generate file 
    generate_vec_patterns(f"{root}/x86/{filename}", els, vecs, lens, feats, blends, elsz, masks)

