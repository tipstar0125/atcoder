#![allow(non_snake_case)]
#![allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};

#[fastout]
fn main() {
    input! {
        N: usize,
        A: [usize; N]
    }
    let mut gen = vec![0; 2 * N + 2];
    for (i, &a) in A.iter().enumerate() {
        gen[2 * (i + 1)] = gen[a] + 1;
        gen[2 * (i + 1) + 1] = gen[a] + 1;
    }

    for &g in &gen[1..] {
        println!("{}", g);
    }
}
