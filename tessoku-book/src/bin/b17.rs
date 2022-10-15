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
        h: [isize; N]
    }

    let mut dp = vec![0; N + 1];
    dp[2] = (h[1] - h[0]).abs();

    for i in 3..=N {
        dp[i] =
            (dp[i - 1] + (h[i - 1] - h[i - 2]).abs()).min(dp[i - 2] + (h[i - 1] - h[i - 3]).abs());
    }

    let mut ans = VecDeque::new();
    let mut place = N;
    ans.push_front(N);
    while place != 1 {
        if dp[place] - dp[place - 1] == (h[place - 1] - h[place - 2]).abs() {
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
