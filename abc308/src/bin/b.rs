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
            M: usize,
            C: [String; N],
            D: [String; M],
            P: [usize; M+1]
        }

        let mut mp: BTreeMap<String, usize> = BTreeMap::new();
        for i in 0..M {
            *mp.entry(D[i].clone()).or_default() = P[i + 1];
        }
        let mut ans = 0_usize;
        for i in 0..N {
            if mp.contains_key(&C[i]) {
                ans += mp[&C[i]];
            } else {
                ans += P[0];
            }
        }
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
