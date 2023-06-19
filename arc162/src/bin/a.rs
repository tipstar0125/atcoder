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
            T: usize
        }

        for _ in 0..T {
            input! {
                N: usize,
                P: [Usize1; N]
            }

            let mut candidate = vec![];
            for i in 0..N {
                if P[i] <= i {
                    candidate.push((P[i], i));
                }
            }
            candidate.sort();
            let mut ans = 1_usize;
            let (mut a, mut b) = candidate[0];
            for i in 1..candidate.len() {
                let (a1, b1) = candidate[i];
                if a < a1 && b < b1 {
                    ans += 1;
                    a = a1;
                    b = b1;
                }
            }
            println!("{}", ans);
        }
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
