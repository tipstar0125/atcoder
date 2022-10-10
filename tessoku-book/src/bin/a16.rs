use proconio::{fastout, input};
use std::cmp;

#[allow(non_snake_case)]
#[fastout]

fn main() {
    input! {
        N: usize,
        A: [usize; N-1],
        B: [usize; N-2]
    }

    let mut dp = vec![0; N + 1];
    dp[2] = A[0];

    for i in 3..=N {
        dp[i] = cmp::min(dp[i - 1] + A[i - 2], dp[i - 2] + B[i - 3]);
    }

    println!("{:?}", dp[N]);
}
