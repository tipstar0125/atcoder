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
            S: [Chars; N]
        }

        let mut SI = vec![];
        for (i, s) in S.iter().enumerate() {
            SI.push((s, i));
        }
        SI.sort();

        fn diff(s1: &Vec<char>, s2: &Vec<char>) -> usize {
            let m = min!(s1.len(), s2.len());
            for i in 0..m {
                if s1[i] != s2[i] {
                    return i;
                }
            }
            m
        }

        let mut ans = vec![0; N];
        for i in 1..N {
            let (s1, idx1) = SI[i - 1];
            let (s2, idx2) = SI[i];
            let n = diff(s1, s2);
            ans[idx1] = max!(ans[idx1], n);
            ans[idx2] = max!(ans[idx2], n);
        }
        println!("{}", ans.iter().join("\n"));
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
