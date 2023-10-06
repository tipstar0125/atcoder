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
            S: usize,
            AB: [(usize, usize); N]
        }

        let mut dp = vec![vec![false; S + 1]; N + 1];
        dp[0][0] = true;
        for i in 1..=N {
            let (a, b) = AB[i - 1];
            for j in 0..=S {
                if j + a <= S {
                    dp[i][j + a] |= dp[i - 1][j];
                }
                if j + b <= S {
                    dp[i][j + b] |= dp[i - 1][j];
                }
            }
        }

        if !dp[N][S] {
            println!("Impossible");
            return;
        }
        let mut ans = vec![];
        let mut now = S;
        for i in (0..N).rev() {
            let (a, b) = AB[i];
            if now >= a && dp[i][now - a] {
                ans.push('A');
                now -= a;
            } else {
                ans.push('B');
                now -= b;
            }
        }
        ans.reverse();
        println!("{}", ans.iter().join(""));
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
