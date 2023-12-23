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
            K: usize,
            A: [usize; K]
        }

        if K % 2 == 0 {
            let mut ans = 0;
            for i in 0..K {
                if i % 2 == 1 {
                    ans += A[i] - A[i - 1];
                }
            }
            println!("{}", ans);
        } else {
            if K == 1 {
                println!("0");
                return;
            }
            let mut a = 0;
            for i in 1..K {
                if i % 2 == 1 {
                    a += A[i + 1] - A[i];
                }
            }
            let mut ans = a;
            for i in 2..K {
                if i % 2 == 0 {
                    a += A[i - 1] - A[i - 2];
                    a -= A[i] - A[i - 1];
                    ans = min!(ans, a);
                }
            }
            println!("{}", ans);
        }
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
