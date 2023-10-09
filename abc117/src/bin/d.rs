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
            K: usize,
            A: [usize; N]
        }

        let MAX = 60;
        let mut dp = vec![vec![-1; 2]; MAX + 1];
        dp[0][0] = 0;
        for i in 1..=MAX {
            let n = (K >> (MAX - i)) & 1;
            let mut cnt = 0;
            for &a in &A {
                if (a >> (MAX - i)) & 1 == 1 {
                    cnt += 1;
                }
            }
            let score0 = (1_isize << (MAX - i)) * cnt; // when bit of X = 0
            let score1 = (1_isize << (MAX - i)) * (N as isize - cnt); // when bit of X = 1

            // smaller -> smaller
            if dp[i - 1][1] != -1 {
                dp[i][1] = max!(dp[i][1], dp[i - 1][1] + max!(score0, score1));
            }
            if dp[i - 1][0] != -1 {
                // not smaller -> smaller
                if n == 1 {
                    dp[i][1] = max!(dp[i][1], dp[i - 1][0] + score0);
                }
                // not smaller -> not smaller
                if n == 1 {
                    dp[i][0] = max!(dp[i][0], dp[i - 1][0] + score1);
                } else {
                    dp[i][0] = max!(dp[i][0], dp[i - 1][0] + score0);
                }
            }
        }

        let ans = max!(dp[MAX][0], dp[MAX][1]);
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
