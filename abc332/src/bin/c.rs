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
            _N: usize,
            M: usize,
            S: Chars
        }

        let mut v = vec![];
        let mut cnt1 = 0;
        let mut cnt2 = 0;
        for s in S {
            if s == '1' {
                cnt1 += 1;
            } else if s == '2' {
                cnt2 += 1;
            } else {
                v.push((cnt1, cnt2));
                cnt1 = 0;
                cnt2 = 0;
            }
        }
        v.push((cnt1, cnt2));
        let mut ans = 0;
        for &(a, b) in &v {
            let mut c = 0;
            if a > M {
                c += a - M;
            }
            c += b;
            ans = max!(ans, c);
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
