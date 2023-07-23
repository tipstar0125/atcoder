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
            H: usize,
            W: usize,
            N: usize,
            AB: [(usize, usize); N]
        }

        let mut S = vec![vec![0; W + 1]; H + 1];
        for &(a, b) in &AB {
            S[a][b] = 1;
        }

        let mut dp = vec![vec![0_usize; W + 1]; H + 1];
        let mut ans = 0_usize;

        for i in 1..=H {
            for j in 1..=W {
                if S[i][j] == 1 {
                    continue;
                }
                let mut min = min!(dp[i - 1][j], dp[i][j - 1]);
                min = min!(min, dp[i - 1][j - 1]);
                dp[i][j] = min + 1;
                ans += dp[i][j];
            }
        }
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
