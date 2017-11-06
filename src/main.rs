extern crate stdsimd;
extern crate faster;
use faster::*;

fn main() {
    let lots_of_84s = (&[-10i8; 128][..]).simd_iter()
        .map(|v| i8s::splat(9) * v.abs() - i8s::splat(4) - i8s::splat(2))
        .scalar_collect::<Vec<i8>>();

    let lots_of_3s = (&[-123.456f32; 128][..]).simd_iter()
        .map(|v| { f32s::splat(9.0) * v.abs().sqrt().rsqrt().ceil().sqrt() -
                   f32s::splat(4.0) - f32s::splat(2.0) })
        .scalar_collect::<Vec<f32>>();

    let lots_of_3s_sc = (&[-123.456f32; 128][..]).iter()
        .map(|v| { 9.0 * v.abs().sqrt().sqrt().recip().ceil().sqrt() -
                   4.0 - 2.0 })
        .collect::<Vec<f32>>();

    println!("{:?}\n{:?}\n{:?}", lots_of_84s, lots_of_3s, lots_of_3s_sc);
}
