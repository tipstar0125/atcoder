#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(unused_macros)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::comparison_chain)]
#![allow(clippy::nonminimal_bool)]
#![allow(clippy::neg_multiply)]
#![allow(dead_code)]
use itertools::Itertools;
use num_integer::gcd;
use ordered_float::OrderedFloat;
use std::cmp::Reverse;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, VecDeque};
use superslice::Ext;
const MOD: usize = 1e9 as usize + 7;

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
            AB: [(usize, usize); N]
        }

        let mut C = vec![];
        for (i, &(a, b)) in AB.iter().enumerate() {
            C.push((a, b, i + 1));
        }
        C.sort_by(|&x, &y| (y.0 * (x.0 + x.1), x.2).cmp(&(x.0 * (y.0 + y.1), y.2)));
        let mut ans = vec![];
        for c in C {
            ans.push(c.2);
        }
        println!("{}", ans.iter().join(" "));
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
