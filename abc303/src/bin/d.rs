#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(unused_macros)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::comparison_chain)]
#![allow(clippy::nonminimal_bool)]
#![allow(clippy::neg_multiply)]
#![allow(dead_code)]
use itertools::Itertools;
use std::cmp::{min, Reverse};
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
            X: usize,
            Y: usize,
            Z: usize,
            S: Chars
        }

        let INF = 1_usize << 60;
        let N = S.len();
        let mut dp = vec![vec![INF; 2]; N + 1];
        dp[0][0] = 0;
        dp[0][1] = Z;

        for i in 1..=N {
            let s = S[i - 1];
            if s == 'a' {
                dp[i][0] = min(dp[i][0], dp[i - 1][0] + X);
                dp[i][1] = min(dp[i][1], dp[i - 1][0] + Y + Z);
                dp[i][1] = min(dp[i][1], dp[i - 1][1] + Y);
                dp[i][0] = min(dp[i][0], dp[i - 1][1] + X + Z);
            } else {
                dp[i][1] = min(dp[i][1], dp[i - 1][1] + X);
                dp[i][0] = min(dp[i][0], dp[i - 1][1] + Y + Z);
                dp[i][0] = min(dp[i][0], dp[i - 1][0] + Y);
                dp[i][1] = min(dp[i][1], dp[i - 1][0] + X + Z);
            }
        }
        let ans = min!(dp[N][0], dp[N][1]);
        println!("{}", ans);
    }
}

#[macro_export]
macro_rules! max {
    ($x: expr) => ($x);
    ($x: expr, $( $y: expr ),+) => {
        std::cmp::max($x, max!($( $y ),+))
    }
}
#[macro_export]
macro_rules! min {
    ($x: expr) => ($x);
    ($x: expr, $( $y: expr ),+) => {
        std::cmp::min($x, min!($( $y ),+))
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
