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
            A: [usize; N]
        }

        let mut s = 0;
        for &a in &A {
            s ^= a;
        }
        if s != 0 {
            println!("-1");
            return;
        }

        let mut mp: BTreeMap<usize, usize> = BTreeMap::new();
        for &a in &A {
            *mp.entry(a).or_default() += 1;
        }
        for (k, v) in mp.iter().rev() {
            if v % 2 == 1 {
                println!("{}", k - 1);
                return;
            }
        }
        println!("0");
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
