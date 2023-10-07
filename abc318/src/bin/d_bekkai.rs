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
            N: usize
        }
        let mut D = vec![vec![0; N]; N];
        for i in 0..N - 1 {
            input! {
                d: [usize; N-i-1]
            }
            for (j, &w) in d.iter().enumerate() {
                D[i][i + j + 1] = w;
                D[i + j + 1][i] = w;
            }
        }

        let MAX = 1_usize << N;
        let mut dp = vec![0; MAX];
        for s in 0..MAX {
            let one_cnt = s.count_ones();
            if one_cnt % 2 != 0 {
                continue;
            }
            let mut zero_vec = vec![];
            for i in 0..N {
                if (s >> i) & 1 == 0 {
                    zero_vec.push(i);
                }
            }
            if zero_vec.len() < 2 {
                continue;
            }
            for comb in zero_vec.iter().combinations(2) {
                let i = *comb[0];
                let j = *comb[1];
                let next = s | 1 << i | 1 << j;
                dp[next] = max!(dp[next], dp[s] + D[i][j]);
            }
        }
        let ans = dp.iter().max().unwrap();
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
