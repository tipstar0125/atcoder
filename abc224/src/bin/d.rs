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
            M: usize,
            UV: [(Usize1, Usize1); M],
            P: [Usize1; 8]
        }

        let mut G = vec![vec![]; 9];
        for &(u, v) in &UV {
            G[u].push(v);
            G[v].push(u);
        }

        let mut group = BTreeSet::new();
        let mut Q = VecDeque::new();
        group.insert(8);
        Q.push_back(8);
        while let Some(pos) = Q.pop_front() {
            for &next in &G[pos] {
                if !group.contains(&next) {
                    group.insert(next);
                    Q.push_back(next);
                }
            }
        }
        let mut init = vec![8; 9];
        for (i, &p) in P.iter().enumerate() {
            init[p] = i as u8;
        }

        let mut status: BTreeMap<Vec<u8>, usize> = BTreeMap::new();
        let mut Q = VecDeque::new();
        *status.entry(init.clone()).or_default() = 0;
        Q.push_back(init);
        while let Some(s) = Q.pop_front() {
            let c = status[&s];
            let mut pos = 0;
            for (i, &p) in s.iter().enumerate() {
                if p == 8 {
                    pos = i;
                    break;
                }
            }
            for &next in &G[pos] {
                let mut ns = s.clone();
                ns.swap(pos, next);
                if !status.contains_key(&ns) || (status.contains_key(&ns) && c + 1 < status[&ns]) {
                    *status.entry(ns.clone()).or_default() = c + 1;
                    Q.push_back(ns);
                }
            }
        }
        let finish = (0..9).collect_vec();
        if status.contains_key(&finish) {
            println!("{}", status[&finish]);
        } else {
            println!("-1");
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
