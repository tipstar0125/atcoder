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
        S: usize,
        A: [usize; N]
    }

    let mut dp = vec![vec![false; S + 1]; N + 1];
    dp[0][0] = true;

    for i in 1..=N {
        let a = A[i - 1];
        for j in 0..=S {
            dp[i][j] |= dp[i - 1][j];
            if dp[i - 1][j] && a + j <= S {
                dp[i][a + j] = true;
            }
        }
    }

    let mut ans = VecDeque::new();
    let mut target = S;
    for i in (1..=N).rev() {
        let a = A[i - 1];
        if dp[i][target] && target >= a && dp[i][target - a] {
            ans.push_front(i);
            target -= a;
        }
    }
    if ans.is_empty() {
        println!("-1");
    } else {
        println!("{}", ans.len());
        println!("{}", ans.iter().join(" "));
    }
}
