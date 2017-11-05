extern crate stdsimd;
extern crate faster;
use stdsimd::simd::u8x32;
use faster::*;

fn main() {
    let lots_of_twos = (&[0u8; 128][..]).simd_iter()
        .map(|v| u8x32::splat(9) * v + u8x32::splat(4) - u8x32::splat(2))
        .scalar_collect::<Vec<u8>>();
    println!("{:?}", lots_of_twos);
}
