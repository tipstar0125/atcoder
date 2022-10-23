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
        W: usize,
        wv: [(usize, usize); N]
    }

    let mut dp = vec![vec![0; W + 1]; N + 1];
    for i in 1..=N {
        let (w, v) = wv[i - 1];
        for j in 0..=W {
            if j < w {
                dp[i][j] = dp[i - 1][j];
            } else {
                dp[i][j] = dp[i - 1][j].max(dp[i - 1][j - w] + v);
            }
        }
    }

    println!("{}", dp[N].iter().max().unwrap());
}
