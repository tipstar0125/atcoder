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
            A: [Usize1; N]
        }

        let mut G = vec![vec![]; N];
        let mut indegs = vec![0; N];
        for (i, &a) in A.iter().enumerate() {
            G[i].push(a);
            indegs[a] += 1;
        }

        let mut v = vec![];
        let mut Q = VecDeque::new();
        for i in 0..N {
            if indegs[i] == 0 {
                v.push(i);
                Q.push_back(i);
            }
        }
        while let Some(pos) = Q.pop_front() {
            for &next in &G[pos] {
                indegs[next] -= 1;
                if indegs[next] == 0 {
                    v.push(next);
                    Q.push_back(next);
                }
            }
        }
        let ans = N - v.len();
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
