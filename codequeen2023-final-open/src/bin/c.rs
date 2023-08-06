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
            S: Usize1,
            T: Usize1,
            UV: [(Usize1, Usize1); N-1]
        }

        let mut G = vec![vec![]; N];
        for &(u, v) in &UV {
            G[u].push(v);
            G[v].push(u);
        }

        let INF = 1_usize << 60;
        let mut Q = VecDeque::new();
        let mut dist = vec![INF; N];
        Q.push_back(S);
        dist[S] = 0;
        while let Some(pos) = Q.pop_front() {
            for &next in &G[pos] {
                if dist[next] == INF {
                    dist[next] = dist[pos] + 1;
                    Q.push_back(next);
                }
            }
        }
        let mut now = T;
        let mut path = vec![T];
        while now != S {
            for &before in &G[now] {
                if dist[before] + 1 == dist[now] {
                    now = before;
                    path.push(before);
                }
            }
        }

        let mut ans = vec![INF; N];
        let mut Q = VecDeque::new();
        for p in path {
            ans[p] = 1;
            Q.push_back(p);
        }

        while let Some(pos) = Q.pop_front() {
            for &next in &G[pos] {
                if ans[next] == INF {
                    ans[next] = ans[pos] + 1;
                    Q.push_back(next);
                }
            }
        }
        println!("{}", ans.iter().join("\n"));
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
