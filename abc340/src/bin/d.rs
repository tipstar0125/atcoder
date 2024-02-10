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
            ABX: [(usize, usize, Usize1); N-1]
        }
        let mut G = vec![vec![]; N];
        for (i, &(a, b, x)) in ABX.iter().enumerate() {
            G[i].push((i + 1, a));
            G[i].push((x, b));
        }
        let INF = 1_usize << 60;
        let mut cost = vec![INF; N];
        let mut Q = BinaryHeap::new();
        cost[0] = 0;
        Q.push((Reverse(cost[0]), 0));
        while let Some((Reverse(d), pos)) = Q.pop() {
            if cost[pos] != d {
                continue;
            }
            for &(next, c) in &G[pos] {
                if cost[pos] + c < cost[next] {
                    cost[next] = cost[pos] + c;
                    Q.push((Reverse(cost[next]), next));
                }
            }
        }
        println!("{}", cost[N - 1]);
    }
}

#[macro_export]
macro_rules! max {
    ($x: expr) => ($x);
    ($x: expr, $( $y: expr ),+) => {
        std::cmp::max($x, max!($( $y ),+))
    }
}
#[macro_export]
macro_rules! min {
    ($x: expr) => ($x);
    ($x: expr, $( $y: expr ),+) => {
        std::cmp::min($x, min!($( $y ),+))
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
