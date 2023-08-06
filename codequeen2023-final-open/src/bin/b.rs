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
            RC: [(Usize1, Usize1); N-1]
        }

        let mut row: BTreeSet<_> = (0..N).collect();
        let mut col: BTreeSet<_> = (0..N).collect();
        let mut naname1: BTreeSet<_> = (0..2 * N).collect();
        let mut naname2: BTreeSet<_> = (0..2 * N).collect();
        for &(r, c) in &RC {
            row.remove(&r);
            col.remove(&c);
            naname1.remove(&(r + c));
            if r < c {
                naname2.remove(&(c - r));
            } else {
                naname2.remove(&(N + r - c));
            }
        }

        for r in row.iter() {
            for c in col.iter() {
                if naname1.contains(&(r + c)) {
                    if r < c && naname2.contains(&(c - r)) {
                        println!("{} {}", r + 1, c + 1);
                        return;
                    }
                    if r >= c && naname2.contains(&(N + r - c)) {
                        println!("{} {}", r + 1, c + 1);
                        return;
                    }
                }
            }
        }
        println!("-1");
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
