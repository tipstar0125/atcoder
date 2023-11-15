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
            mut K: usize,
            mut A: [usize; N]
        }

        let mut ok = 0;
        let mut ng = 1e18 as usize;

        fn f(m: usize, K: usize, A: &[usize]) -> bool {
            let mut S = 0_usize;
            for &a in A {
                S += min!(m, a);
            }
            S <= K
        }

        while ng - ok > 1 {
            let m = (ok + ng) / 2;
            if f(m, K, &A) {
                ok = m;
            } else {
                ng = m;
            }
        }

        for i in 0..N {
            if A[i] < ok {
                K -= A[i];
                A[i] = 0;
            } else {
                A[i] -= ok;
                K -= ok;
            }
        }
        for i in 0..N {
            if K > 0 && A[i] > 0 {
                A[i] -= 1;
                K -= 1;
            }
        }
        assert!(K == 0);
        println!("{}", A.iter().join(" "));
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
