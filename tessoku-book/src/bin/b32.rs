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
        K: usize,
        a: [usize; K]
    }

    let mut dp = vec![false; N + 1];
    for i in 0..=N {
        for &x in &a {
            if i >= x && !dp[i - x] {
                dp[i] = true;
            }
        }
    }

    if dp[N] {
        println!("First");
    } else {
        println!("Second");
    }
}
