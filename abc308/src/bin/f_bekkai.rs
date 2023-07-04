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
            P: [usize; N],
            L: [usize; M],
            D: [usize; M],
        }

        let mut DL = vec![];
        for i in 0..M {
            DL.push((D[i], L[i]));
        }
        DL.sort();
        DL.reverse();

        let mut set = BTreeSet::new();
        for i in 0..N {
            set.insert((P[i], i));
        }
        let mut ans: usize = P.iter().sum();
        for &(d, l) in &DL {
            let p = set.range((l, 0)..).next();
            if p.is_some() {
                let pp = *p.unwrap();
                set.remove(&pp);
                ans -= d;
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
