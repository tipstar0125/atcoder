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
        A: usize,
        B: usize
    }

    let mut dp = vec![false; N + 1];
    for i in 0..=N {
        if i >= A && !dp[i - A] {
            dp[i] = true;
        }
        if i >= B && !dp[i - B] {
            dp[i] = true;
        }
    }

    if dp[N] {
        println!("First");
    } else {
        println!("Second");
    }
}
