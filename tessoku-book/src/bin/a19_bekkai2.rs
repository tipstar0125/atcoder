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
        let w = wv[i - 1].0;
        let v = wv[i - 1].1;
        for j in 0..=W {
            dp[i][j] = dp[i][j].max(dp[i - 1][j]);
            if (j == 0 || dp[i - 1][j] != 0) && j + w <= W {
                dp[i][j + w] = dp[i - 1][j] + v;
            }
        }
    }

    println!("{}", dp[N].iter().max().unwrap());
}
