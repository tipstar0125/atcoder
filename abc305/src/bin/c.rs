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
            H: usize,
            W: usize,
            S: [Chars; H]
        }

        let mut v = vec![];
        for i in 0..H {
            for j in 0..W {
                if S[i][j] == '#' {
                    v.push((i, j));
                }
            }
        }

        let mut row: BTreeMap<usize, usize> = BTreeMap::new();
        let mut col: BTreeMap<usize, usize> = BTreeMap::new();
        for &(r, c) in &v {
            *row.entry(r).or_default() += 1;
            *col.entry(c).or_default() += 1;
        }
        let mut h = 0;
        let mut w = 0;
        for (_, a) in row.iter() {
            h = max!(h, *a);
        }
        for (_, a) in col.iter() {
            w = max!(w, *a);
        }
        let mut x = 0;
        let mut y = 0;
        for (k, a) in row.iter() {
            if h != *a {
                x = *k;
            }
        }
        for (k, a) in col.iter() {
            if w != *a {
                y = *k;
            }
        }
        println!("{} {}", x + 1, y + 1);
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
