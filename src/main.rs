extern crate stdsimd;
extern crate faster;
use faster::*;

fn main() {
    let lots_of_84s = (&[-10i8; 128][..]).simd_iter()
        .map(|v| i8s::splat(9) * v.abs() - i8s::splat(4) - i8s::splat(2))
        .scalar_collect::<Vec<i8>>();
    println!("{:?}", lots_of_84s);
}
