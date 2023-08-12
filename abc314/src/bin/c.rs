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
            S: Chars,
            C: [Usize1; N]
        }

        let mut T = vec![VecDeque::new(); M];

        for i in 0..N {
            let s = S[i];
            let c = C[i];
            T[c].push_back(s);
        }

        for i in 0..M {
            if T[i].is_empty() {
                continue;
            }
            let tail = T[i].pop_back().unwrap();
            T[i].push_front(tail);
        }

        let mut ans = vec![];
        for i in 0..N {
            let c = C[i];
            ans.push(T[c].pop_front().unwrap());
        }
        println!("{}", ans.iter().join(""));
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
