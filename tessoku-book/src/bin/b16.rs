use proconio::{fastout, input};
use std::cmp;

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize,
        h: [isize; N]
    }

    let mut dp = vec![0; N + 1];
    dp[2] = (h[1] - h[0]).abs();

    for i in 3..=N {
        dp[i] = cmp::min(
            (h[i - 1] - h[i - 2]).abs() + dp[i - 1],
            (h[i - 1] - h[i - 3]).abs() + dp[i - 2],
        );
    }
    println!("{:?}", dp[N]);
}
