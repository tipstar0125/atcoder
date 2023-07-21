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
            T: usize,
            M: usize,
            AB: [(Usize1, Usize1); M]
        }

        let MAX = 1_usize << N;
        let mut hate_teams = vec![false; MAX];
        for &(a, b) in &AB {
            let mask = (1_usize << a) | (1_usize << b);
            for i in 0..MAX {
                if mask == (mask & i) {
                    hate_teams[i] = true;
                }
            }
        }

        let mut dp = vec![vec![0_usize; T + 1]; MAX];
        dp[0][0] = 1;

        for i in 0..MAX {
            let c = (i + 1) | i;
            let mut j = c;
            while j < MAX {
                let x = j ^ i;
                if !hate_teams[x] {
                    for k in 0..T {
                        dp[j][k + 1] += dp[i][k];
                    }
                }
                j = (j + 1) | c;
            }
        }

        println!("{}", dp[MAX - 1][T]);
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
