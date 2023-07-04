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
            mut M: usize,
            A: [usize; N]
        }

        let mut B = A[2..].iter().collect_vec();
        B.sort();
        let a0 = A[0];
        let a1 = A[1];

        if a0 <= a1 {
            let mut mp: BTreeMap<usize, usize> = BTreeMap::new();
            let mut C = vec![];
            for &b in &B {
                if *b < a0 || *b > a1 {
                    *mp.entry(*b).or_default() += 1;
                    C.push(*b);
                } else {
                    M -= 1;
                    if M == 0 {
                        println!("0");
                        return;
                    }
                }
            }
            C.sort();
            C.dedup();

            let mut ans = 1_usize << 60;
            let mut s = mp[&C[0]];
            let mut r = 0_usize;
            for l in 0..C.len() {
                while r < C.len() - 1 && s < M {
                    r += 1;
                    s += mp[&C[r]];
                }
                let c0 = C[l];
                let c1 = C[r];
                if s >= M {
                    if c0 < a0 && a1 < c1 {
                        ans = min!(ans, a0 - c0 + c1 - a1);
                    } else if c1 < a0 {
                        ans = min!(ans, a0 - c0);
                    } else {
                        ans = min!(ans, c1 - a1);
                    }
                }
                s -= mp[&C[l]];
            }
            println!("{}", ans);
        } else {
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
                    let d0 = a0 as isize - *b0 as isize;
                    let d0 = d0.abs() as usize;
                    let d1 = a1 as isize - *b1 as isize;
                    let d1 = d1.abs() as usize;
                    ans = min!(ans, d0 + d1);
                }
                s -= mp[B[l]];
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
