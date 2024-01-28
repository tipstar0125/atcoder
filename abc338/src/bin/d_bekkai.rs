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
            X: [Usize1; M]
        }

        let mut ans = 0;
        let mut imos = vec![0_isize; N + 1];
        for i in 1..M {
            let mut u = X[i - 1];
            let mut v = X[i];
            if u > v {
                std::mem::swap(&mut u, &mut v);
            }
            let d = v - u;
            if d <= N - d {
                ans += d;
                let cost = N - d - d;
                imos[u] += cost as isize;
                imos[v] -= cost as isize;
            } else {
                ans += N - d;
                let cost = d - (N - d);
                imos[0] += cost as isize;
                imos[u] -= cost as isize;
                imos[v] += cost as isize;
                imos[N] -= cost as isize;
            }
        }
        let mut S = vec![0; N + 1];
        for i in 0..=N {
            if i == 0 {
                S[i] = imos[i];
            } else {
                S[i] = S[i - 1] + imos[i];
            }
        }
        let mn = *S[..N].iter().min().unwrap() as usize;
        println!("{}", ans + mn);
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
