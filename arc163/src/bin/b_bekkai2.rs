#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(unused_macros)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::comparison_chain)]
#![allow(clippy::nonminimal_bool)]
#![allow(clippy::neg_multiply)]
#![allow(dead_code)]
use itertools::Itertools;
use std::cmp::{Reverse, min};
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
            mut M: usize,
            A: [usize; N]
        }

        let mut B = A[2..].iter().collect_vec();
        B.sort();
        let a0 = A[0];
        let a1 = A[1];

        let mut mp: BTreeMap<usize, usize> = BTreeMap::new();
        for &b in &B {
            *mp.entry(*b).or_default() += 1;
        }
        B.dedup();

        let mut ans = 1_usize << 60;
        let mut s = mp[B[0]];
        let mut r = 0_usize;
        for l in 0..B.len() {
            while r < B.len() - 1 && s < M {
                r += 1;
                s += mp[B[r]];
            }
            let b0 = B[l];
            let b1 = B[r];
            if s >= M {
                let mut d = 0;
                if a0 > *b0 {
                    d += a0 - b0;
                }
                if a1 < *b1 {
                    d += b1 - a1;
                }
                ans = min!(ans, d);
            }
            s -= mp[B[l]];
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
