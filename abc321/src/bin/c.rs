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
            K: usize
        }
        let mut v = vec![];
        for i in 1..=10 {
            for nn in (0..=9).combinations(i) {
                let mut s = 0;
                for (j, &n) in nn.iter().enumerate() {
                    s += n * 10_usize.pow(j as u32);
                }
                if s != 0 {
                    v.push(s);
                }
            }
        }
        v.sort();
        v.dedup();
        println!("{}", v[K - 1]);
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
