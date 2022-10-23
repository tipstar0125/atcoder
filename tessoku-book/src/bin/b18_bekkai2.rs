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

    if !dp[N][S] {
        println!("-1");
    } else {
        let mut ans = VecDeque::new();
        let mut place = S;
        for i in (1..=N).rev() {
            let a = A[i - 1];
            if !dp[i - 1][place] {
                ans.push_front(i);
                place -= a;
            }
        }
        println!("{}", ans.len());
        println!("{}", ans.iter().join(" "));
    }
}
