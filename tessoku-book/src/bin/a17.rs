#![allow(non_snake_case)]
#![allow(unused_imports)]
use std::collections::VecDeque;

use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};

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
        dp[i] = (dp[i - 1] + A[i - 2]).min(dp[i - 2] + B[i - 3]);
    }

    let mut ans = VecDeque::new();
    ans.push_front(N);
    let mut place = N;
    while place != 1 {
        if dp[place] - dp[place - 1] == A[place - 2] {
            ans.push_front(place - 1);
            place -= 1;
        } else {
            ans.push_front(place - 2);
            place -= 2;
        }
    }
    println!("{}", ans.len());
    println!("{}", ans.iter().join(" "));
}
