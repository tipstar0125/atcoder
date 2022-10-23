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

    let mut dp = vec![vec![false; W + 1]; N + 1];
    let mut dp_v = vec![vec![0; W + 1]; N + 1];
    dp[0][0] = true;
    for i in 1..=N {
        let w = wv[i - 1].0;
        let v = wv[i - 1].1;
        for j in 0..=W {
            dp[i][j] |= dp[i - 1][j];
            dp_v[i][j] = dp_v[i][j].max(dp_v[i - 1][j]);
            if dp[i - 1][j] && j + w <= W {
                dp[i][j + w] = true;
                dp_v[i][j + w] = dp_v[i - 1][j] + v;
            }
        }
    }

    println!("{}", dp_v[N].iter().max().unwrap());
}
