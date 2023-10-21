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
            N: i128
        }

        fn f(a: i128, b: i128, N: i128) -> bool {
            a * a * a + a * a * b + a * b * b + b * b * b >= N
        }

        let mut ans = 1_i128 << 120;
        for a in 0..=1e6 as i128 {
            let mut ng = -1_i128;
            let mut ok = 1e6 as i128;
            while ok - ng > 1 {
                let m = (ok + ng) / 2;
                if f(a, m, N) {
                    ok = m;
                } else {
                    ng = m;
                }
            }
            let b = ok;
            let x = a * a * a + a * a * b + a * b * b + b * b * b;
            ans = min!(ans, x);
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
