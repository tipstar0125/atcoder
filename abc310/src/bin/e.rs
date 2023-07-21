#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(unused_macros)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::comparison_chain)]
#![allow(clippy::nonminimal_bool)]
#![allow(clippy::neg_multiply)]
#![allow(dead_code)]
use itertools::Itertools;
use std::cmp::Reverse;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, VecDeque};
use superslice::Ext;

use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};

#[derive(Default)]
struct Solver {}
impl Solver {
    #[fastout]
    fn solve(&mut self) {
        input! {
            N: usize,
            S: Chars
        }

        let mut ans = 0;
        let mut dp = vec![vec![0_usize; 2]; N + 1];

        for i in 1..=N {
            let s = S[i - 1];
            if s == '0' {
                dp[i][0] = 1;
                dp[i][1] = dp[i - 1][0] + dp[i - 1][1];
            } else {
                dp[i][0] = dp[i - 1][1];
                dp[i][1] = dp[i - 1][0];
                dp[i][1] += 1;
            }
            ans += dp[i][1];
        }
        println!("{}", ans);
    }
}

fn main() {
    std::thread::Builder::new()
        .stack_size(128 * 1024 * 1024)
        .spawn(|| Solver::default().solve())
        .unwrap()
        .join()
        .unwrap();
}
