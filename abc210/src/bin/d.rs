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
            C: usize,
            mut A: [[usize; W]; H]
        }

        let INF = 1_usize << 60;
        let mut ans = INF;
        let mut dp = vec![vec![INF; W]; H];

        for _ in 0..2 {
            for i in 0..H {
                for j in 0..W {
                    let mut x = INF;
                    if i > 0 {
                        x = min!(x, dp[i - 1][j] + C);
                    }
                    if j > 0 {
                        x = min!(x, dp[i][j - 1] + C);
                    }
                    ans = min!(ans, x + A[i][j]);
                    x = min!(x, A[i][j]);
                    dp[i][j] = x;
                }
                A[i].reverse();
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
