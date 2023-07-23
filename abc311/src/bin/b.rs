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
            D: usize,
            S: [Chars; N]
        }

        let mut ans = 0_usize;
        let mut v = vec![];
        for i in 0..D {
            let mut ok = 1;
            for j in 0..N {
                if S[j][i] == 'x' {
                    ok = 0;
                }
            }
            v.push(ok);
        }

        let rle = run_length_encoding(v);
        for &(k, v) in &rle {
            if k == 1 {
                ans = max!(ans, v);
            }
        }
        println!("{}", ans);
    }
}

fn run_length_encoding<T: Eq>(v: Vec<T>) -> Vec<(T, usize)> {
    let mut v = v.into_iter().map(|v| (v, 1)).collect::<Vec<_>>();
    v.dedup_by(|a, b| {
        a.0 == b.0 && {
            b.1 += a.1;
            true
        }
    });
    v
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
