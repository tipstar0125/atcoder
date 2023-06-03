#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(unused_macros)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::comparison_chain)]
#![allow(clippy::nonminimal_bool)]
#![allow(clippy::neg_multiply)]
#![allow(dead_code)]
use itertools::Itertools;
use std::cmp::{min, Reverse};
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
            W: usize,
            H: usize,
            N: usize,
            PQ: [(usize, usize); N],
            a: usize,
            mut A: [usize; a],
            b: usize,
            mut B: [usize; b]
        }

        A.push(W);
        B.push(H);
        let MAX = (a + 1) * (b + 1);

        let mut mp: BTreeMap<(usize, usize), usize> = BTreeMap::new();
        for &(p, q) in &PQ {
            let pos_x = A.lower_bound(&p);
            let pos_y = B.lower_bound(&q);
            *mp.entry((pos_x, pos_y)).or_default() += 1;
        }
        let INF = 1_usize << 60;
        let mut m = INF;
        let mut M = 0;
        for (_k, &v) in mp.iter() {
            m = min!(m, v);
            M = max!(M, v);
        }
        if mp.len() < MAX {
            m = 0;
        }
        println!("{} {}", m, M);
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
