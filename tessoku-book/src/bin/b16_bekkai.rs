use proconio::{fastout, input};
use std::cmp;

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize,
        h: [isize; N]
    }

    let INF = 1_isize << 60;
    let mut dp = vec![INF; N + 1];
    dp[1] = 0;

    for i in 2..=N {
        dp[i] = cmp::min(dp[i], dp[i - 1] + (h[i - 1] - h[i - 2]).abs());
        if i >= 3 {
            dp[i] = cmp::min(dp[i], dp[i - 2] + (h[i - 1] - h[i - 3]).abs());
        }
    }
    println!("{:?}", dp[N]);
}
