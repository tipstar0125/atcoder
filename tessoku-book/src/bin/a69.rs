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
            C: [Chars; N]
        }
        let mut flow = MaximumFlow::new(2 * N + 2);
        for i in 0..N {
            for j in 0..N {
                if C[i][j] == '#' {
                    flow.add(i, N + j, 1);
                }
            }
        }
        for i in 0..N {
            flow.add(2 * N, i, 1);
            flow.add(N + i, 2 * N + 1, 1);
        }
        let ans = flow.max_flow(2 * N, 2 * N + 1);
        println!("{}", ans);
    }
}

#[derive(Debug, Clone)]
struct Edge {
    to: usize,
    cap: usize,
    rev: usize,
}

#[derive(Debug, Clone)]
struct MaximumFlow {
    size: usize,
    G: Vec<Vec<Edge>>,
}

impl MaximumFlow {
    fn new(n: usize) -> Self {
        MaximumFlow {
            size: n,
            G: vec![vec![]; n],
        }
    }
    fn add(&mut self, u: usize, v: usize, c: usize) {
        let u_size = self.G[u].len();
        let v_size = self.G[v].len();
        self.G[u].push(Edge {
            to: v,
            cap: c,
            rev: v_size,
        });
        self.G[v].push(Edge {
            to: u,
            cap: 0,
            rev: u_size,
        });
    }
    fn dfs(&mut self, pos: usize, goal: usize, F: usize, used: &mut Vec<bool>) -> usize {
        if pos == goal {
            return F;
        }
        used[pos] = true;

        for i in 0..self.G[pos].len() {
            if self.G[pos][i].cap == 0 {
                continue;
            }
            if used[self.G[pos][i].to] {
                continue;
            }
            let flow = self.dfs(self.G[pos][i].to, goal, F.min(self.G[pos][i].cap), used);
            if flow > 0 {
                self.G[pos][i].cap -= flow;
                let to = self.G[pos][i].to;
                let rev = self.G[pos][i].rev;
                self.G[to][rev].cap += flow;
                return flow;
            }
        }
        0
    }
    fn max_flow(&mut self, s: usize, t: usize) -> usize {
        let mut total_flow = 0;
        loop {
            let mut used = vec![false; self.size];
            let flow = self.dfs(s, t, std::usize::MAX, &mut used);
            if flow == 0 {
                break;
            }
            total_flow += flow;
        }
        total_flow
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
