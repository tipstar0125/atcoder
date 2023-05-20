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
            M: usize,
            D: isize,
            mut A: [isize; N],
            mut B: [isize; M]
        }

        A.sort();
        B.sort();

        let mut ans = -1;
        for &a in &A {
            let pos = B.upper_bound(&(a + D));
            if pos == 0 {
                continue;
            }
            if (B[pos - 1] - a).abs() <= D {
                ans = max!(ans, a + B[pos - 1]);
            }
        }
        for &b in &B {
            let pos = A.upper_bound(&(b + D));
            if pos == 0 {
                continue;
            }
            if (A[pos - 1] - b).abs() <= D {
                ans = max!(ans, b + A[pos - 1]);
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
