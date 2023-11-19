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
            Q: usize,
            C: [usize; N]
        }
        let mut box_set = vec![BTreeSet::new(); N];
        for (i, &c) in C.iter().enumerate() {
            box_set[i].insert(c);
        }
        for _ in 0..Q {
            input! {
                a: Usize1,
                b: Usize1,
            }
            if box_set[a].len() < box_set[b].len() {
                for &x in box_set[a].clone().iter() {
                    box_set[b].insert(x);
                }
                box_set[a].clear();
            } else {
                for &x in box_set[b].clone().iter() {
                    box_set[a].insert(x);
                }
                box_set[b].clear();
                box_set.swap(a, b);
            }
            println!("{}", box_set[b].len());
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
