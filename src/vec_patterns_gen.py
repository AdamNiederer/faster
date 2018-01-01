# This file is part of faster, the SIMD library for humans.
# Copyright 2017 Adam Niederer <adam.niederer@gmail.com>

# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http:#mozilla.org/MPL/2.0/.

# Who needs procedural macros when you have code generators?

vecs = ["u8x64", "u8x32", "u8x16", "i8x64", "i8x32", "i8x16", "u16x32", "u16x16", "u16x8", "i16x32", "i16x16", "i16x8", "u32x16", "u32x8", "u32x4", "i32x16", "i32x8", "i32x4", "f32x16", "f32x8", "f32x4", "u64x8", "u64x4", "u64x2", "i64x8", "i64x4", "i64x2", "f64x8", "f64x4", "f64x2"]
lens = [int(v.split("x")[1]) for v in vecs]
els = [v.split("x")[0] for v in vecs]

print("""// This file is part of faster, the SIMD library for humans.
// Copyright 2017 Adam Niederer <adam.niederer@gmail.com>

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

// This file is machine-generated. See vec_patterns_gen.py for more info.""")

print("""
use vecs::*;

pub trait PackedPattern : Packed {
    fn halfs(hi: Self::Scalar, lo: Self::Scalar) -> Self;
    fn interleave(hi: Self::Scalar, lo: Self::Scalar) -> Self;
    fn partition(hi: Self::Scalar, lo: Self::Scalar, off: usize) -> Self;
}
""")

for e, v, l in zip(els, vecs, lens):
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
    print(f"    }}\n")

    # Generate partition
    print(f"    #[inline(always)]")
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
    print(f"}}\n")
