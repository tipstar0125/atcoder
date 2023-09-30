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
            P: [[usize; W]; H]
        }

        let mut C = vec![vec![0; 1_usize << H]; H * W + 1];
        for j in 0..W {
            let mut mp: BTreeMap<usize, usize> = BTreeMap::new();
            for i in 0..H {
                let x = P[i][j];
                *mp.entry(x).or_default() |= 1_usize << i;
            }
            for (x, v) in mp.iter() {
                for k in 1..1_usize << H {
                    if v & k == k {
                        C[*x][k] += 1;
                    }
                }
            }
        }

        let mut ans = 0;
        for i in 1..=H * W {
            for j in 1..1_usize << H {
                ans = max!(ans, C[i][j] * j.count_ones());
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
