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
            XY: [(usize, isize); N]
        }

        let INF = 1_isize << 60;
        let mut dp = vec![vec![-INF; 2]; N + 1];
        dp[0][0] = 0;
        for i in 1..=N {
            let (x, y) = XY[i - 1];
            dp[i][0] = dp[i - 1][0];
            dp[i][1] = dp[i - 1][1];
            if x == 0 {
                dp[i][0] = max!(dp[i][0], dp[i - 1][0] + y);
                dp[i][0] = max!(dp[i][0], dp[i - 1][1] + y);
            } else {
                dp[i][1] = max!(dp[i][1], dp[i - 1][0] + y);
            }
        }
        println!("{}", max!(dp[N][0], dp[N][1]));
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
