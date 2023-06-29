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
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, VecDeque};
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
            K: usize,
            Q: usize,
            XY: [(Usize1, usize); Q]
        }

        let mut A = vec![0; N];
        let mut X = BTreeSet::new();
        let mut Y = BTreeSet::new();
        for i in 0..K {
            X.insert((0, i));
        }
        for i in K..N {
            Y.insert((0, i));
        }

        let mut ans = 0;

        for &(x, y) in &XY {
            let before = A[x];
            A[x] = y;
            if before != y {
                if X.contains(&(before, x)) {
                    if Y.is_empty() || y > Y.iter().next_back().unwrap().0 {
                        X.insert((y, x));
                        ans += y;
                    } else {
                        let Y_max = *Y.iter().next_back().unwrap();
                        X.insert(Y_max);
                        Y.insert((y, x));
                        Y.remove(&Y_max);
                        ans += Y_max.0;
                    }
                    X.remove(&(before, x));
                    ans -= before;
                } else {
                    if y < X.iter().next().unwrap().0 {
                        Y.insert((y, x));
                    } else {
                        let X_min = *X.iter().next().unwrap();
                        Y.insert(X_min);
                        X.insert((y, x));
                        X.remove(&X_min);
                        ans += y;
                        ans -= X_min.0;
                    }
                    Y.remove(&(before, x));
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
