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
            Q: usize
        }

        let mut A = VecDeque::new();
        let mut B = BinaryHeap::new();
        for _ in 0..Q {
            input! {
                t: usize
            }
            if t == 1 {
                input! {
                    x: usize
                }
                A.push_back(x);
            } else if t == 2 {
                if B.is_empty() {
                    let ans = A.pop_front().unwrap();
                    println!("{}", ans);
                } else {
                    let Reverse(ans) = B.pop().unwrap();
                    println!("{}", ans);
                }
            } else {
                while !A.is_empty() {
                    B.push(Reverse(A.pop_back().unwrap()));
                }
            }
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
