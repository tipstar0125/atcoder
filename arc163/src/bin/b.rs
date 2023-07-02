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
        let mut a0 = A[0];
        let mut a1 = A[1];
        if a0 > a1 {
            std::mem::swap(&mut a0, &mut a1);
        }

        let mut C: BTreeMap<usize, usize> = BTreeMap::new();
        let mut D = vec![];
        for &b in &B {
            if *b < a0 || *b > a1 {
                *C.entry(*b).or_default() += 1;
                D.push(*b);
            } else {
                M -= 1;
                if M == 0 {
                    println!("0");
                    return;
                }
            }
        }
        D.sort();
        D.dedup();

        let mut ans = 1_usize << 60;
        let mut s = C[&D[0]];
        let mut r = 0_usize;
        // println!("{:?}", C);
        // println!("{:?}", D);
        for l in 0..D.len() {
            // println!("{} {} {} {}", r, D.len() - 1, s, M);
            while r < D.len() - 1 && s < M {
                r += 1;
                s += C[&D[r]];
                // println!("s: {}", s);
            }
            let c0 = D[l];
            let c1 = D[r];
            // println!("c0: {} c1: {} s: {}", c0, c1, s);
            if s >= M {
                if c0 < a0 && a1 < c1 {
                    ans = min!(ans, a0 - c0 + c1 - a1);
                } else if c1 < a0 {
                    ans = min!(ans, a0 - c0);
                } else {
                    ans = min!(ans, c1 - a1);
                }
            }
            s -= C[&D[l]];
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
