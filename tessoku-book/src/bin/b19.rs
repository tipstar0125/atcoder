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

    let V_MAX = 1000 * N;
    let INF = 1 << 60;
    let mut dp = vec![vec![INF; V_MAX + 1]; N + 1];
    dp[0][0] = 0;
    for i in 1..=N {
        let (w, v) = wv[i - 1];
        for j in 0..=V_MAX {
            if j < v {
                dp[i][j] = dp[i - 1][j];
            } else {
                dp[i][j] = dp[i - 1][j].min(dp[i - 1][j - v] + w);
            }
        }
    }

    let mut ans = 0;
    for (i, &x) in dp[N].iter().enumerate() {
        if x <= W {
            ans = i;
        }
    }
    println!("{}", ans);
}
