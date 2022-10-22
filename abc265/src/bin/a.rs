#![allow(non_snake_case)]
#![allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};

#[fastout]
fn main() {
    input! {
        X: usize,
        Y: usize,
        N: usize
    }

    let x = X as f64;
    let y = Y as f64;
    if x > y / 3.0 {
        let ans = (N / 3) * Y + (N % 3) * X;
        println!("{}", ans);
    } else {
        println!("{}", N * X);
    }
}
