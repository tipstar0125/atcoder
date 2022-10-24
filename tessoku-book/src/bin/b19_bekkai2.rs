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
    let mut dp = vec![vec![0; V_MAX + 1]; N + 1];
    for i in 1..=N {
        let (w, v) = wv[i - 1];
        for j in 0..=V_MAX {
            if j < v {
                dp[i][j] = dp[i - 1][j];
            } else {
                let k = dp[i - 1][j - v] + w;
                if (j - v == 0 || dp[i - 1][j - v] != 0) && k <= W {
                    dp[i][j] = k;
                    if dp[i - 1][j] != 0 {
                        dp[i][j] = dp[i - 1][j].min(k);
                    } else {
                        dp[i][j] = k;
                    }
                } else {
                    dp[i][j] = dp[i - 1][j];
                }
            }
        }
    }

    let mut ans = 0;
    for (i, &x) in dp[N].iter().enumerate() {
        if x != 0 {
            ans = i;
        }
    }
    println!("{}", ans);
}
