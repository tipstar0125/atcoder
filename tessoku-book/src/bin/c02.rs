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
        mut A: [usize; N]
    }

    A.sort();
    println!("{}", A[A.len() - 1] + A[A.len() - 2]);
}
