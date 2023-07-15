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
            S: [Chars; N]
        }

        let mut set1 = BTreeSet::new();
        let mut set2 = BTreeSet::new();
        for i in 0..N {
            let s = S[i].clone();
            let mut s_rev = S[i].clone();
            s_rev.reverse();
            if s == s_rev {
                set1.insert(s);
            } else {
                set2.insert(s);
                set2.insert(s_rev);
            }
        }
        let ans = set1.len() + set2.len() / 2;
        println!("{}", ans);
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
