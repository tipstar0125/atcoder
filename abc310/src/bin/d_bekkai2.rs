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

        let mut dp = vec![0_usize; MAX];
        dp[MAX - 1] = 1;
        for t in 0..T {
            let mut ndp = vec![0_usize; MAX];
            for i in 1..MAX {
                let mut ones = vec![];
                for j in 0..N {
                    if (i >> j) & 1 == 1 {
                        ones.push(j);
                    }
                }
                let pattern = make_pattern(ones);
                for &p in pattern[1..].iter() {
                    if !hate_teams[p] {
                        ndp[i ^ p] += dp[i];
                    }
                }
            }
            dp = ndp;
            for i in 0..MAX {
                dp[i] /= t + 1;
            }
        }
        println!("{}", dp[0]);
    }
}

fn make_pattern(ones: Vec<usize>) -> Vec<usize> {
    let mut pattern = vec![0; 1_usize << ones.len()];
    for i in 1..1 << ones.len() {
        let mut s = 0;
        for j in 0..ones.len() {
            if (i >> j) & 1 == 1 {
                s |= 1_usize << ones[j];
            }
        }
        pattern[i] = s;
    }
    pattern
}

fn main() {
    std::thread::Builder::new()
        .stack_size(128 * 1024 * 1024)
        .spawn(|| Solver::default().solve())
        .unwrap()
        .join()
        .unwrap();
}
