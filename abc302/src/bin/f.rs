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
        }

        let mut G = vec![vec![]; N + M];
        for i in 0..N {
            input! {
                A: usize,
                S: [Usize1; A]
            }
            for &s in &S {
                G[i].push(s + N);
                G[s + N].push(i);
            }
        }

        let mut Q = VecDeque::new();
        let INF = 1_usize << 60;
        let mut dist = vec![INF; N + M];
        Q.push_back(N);
        dist[N] = 0;
        while !Q.is_empty() {
            let pos = Q.pop_front().unwrap();
            for &next in &G[pos] {
                if dist[pos] + 1 < dist[next] {
                    dist[next] = dist[pos] + 1;
                    Q.push_back(next);
                }
            }
        }

        if dist[N + M - 1] >= INF {
            println!("-1");
        } else {
            let ans = dist[N + M - 1] / 2 - 1;
            println!("{}", ans);
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
