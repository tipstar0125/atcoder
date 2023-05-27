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

        let mut A = vec![vec![0; 2 * N]; 2 * N];
        for i in 0..2 * N - 1 {
            for j in i + 1..2 * N {
                input! {
                    a: usize
                }
                A[i][j] = a;
            }
        }

        let MAX = 1_usize << (2 * N);
        let mut dp = vec![BTreeSet::new(); MAX];
        for bit in 0..MAX {
            if bit.count_ones() == 2 {
                let mut pair = vec![];
                for j in 0..2 * N {
                    if (bit >> j) & 1 == 1 {
                        pair.push(j);
                    }
                }
                dp[bit].insert(A[pair[0]][pair[1]]);
            }
        }

        for s in 0..MAX {
            let one_cnt = s.count_ones();
            if one_cnt > 2 && one_cnt % 2 == 0 {
                let mut ones = vec![];
                for j in 0..2 * N {
                    if (s >> j) & 1 == 1 {
                        ones.push(j);
                    }
                }

                for v in ones.iter().combinations(2) {
                    let mut before_s = 1_usize << v[0];
                    before_s += 1_usize << v[1];
                    let x = *dp[before_s].iter().next().unwrap();
                    for y in dp[s ^ before_s].clone().iter() {
                        dp[s].insert(x ^ y);
                    }
                }
            }
        }

        let ans = dp[MAX - 1].iter().max().unwrap();
        println!("{}", ans);
    }
}

pub trait ChangeMinMax {
    fn chmin(&mut self, x: Self) -> bool;
    fn chmax(&mut self, x: Self) -> bool;
}

impl<T: PartialOrd> ChangeMinMax for T {
    fn chmin(&mut self, x: Self) -> bool {
        *self > x && {
            *self = x;
            true
        }
    }
    fn chmax(&mut self, x: Self) -> bool {
        *self < x && {
            *self = x;
            true
        }
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
