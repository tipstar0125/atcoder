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
            X: usize,
            M: usize
        }

        let mut dp = vec![vec![(0, 0); M]; 60];
        for i in 0..M {
            let x = (i * i) % M;
            dp[0][i] = (x, i);
        }

        for i in 1..34 {
            for j in 0..M {
                let s = dp[i - 1][j].1 + dp[i - 1][dp[i - 1][j].0].1;
                dp[i][j] = (dp[i - 1][dp[i - 1][j].0].0, s);
            }
        }

        let mut now = X;
        let mut ans = 0;
        for i in (0..60).rev() {
            if (N >> i) & 1 == 1 {
                ans += dp[i][now].1;
                now = dp[i][now].0;
            }
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
