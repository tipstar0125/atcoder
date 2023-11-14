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
            AB: [(usize, usize); M]
        }

        let mut edge = vec![vec![]; N];
        for &(a, b) in &AB {
            edge[a].push(b);
        }
        let mut scc = StronglyConnectedComponent::new(&edge);
        let groups = scc.scc();
        println!("{}", groups.len());
        for g in groups {
            println!("{} {}", g.len(), g.iter().join(" "));
        }
    }
}

#[derive(Debug, Clone)]
struct StronglyConnectedComponent<'a> {
    size: usize,
    edge: &'a Vec<Vec<usize>>,
    visited: Vec<bool>,
    order: Vec<usize>,
    low: Vec<usize>,
    ids: Vec<usize>,
    group_size: usize,
}

impl<'a> StronglyConnectedComponent<'a> {
    fn new(edge: &'a Vec<Vec<usize>>) -> Self {
        let size = edge.len();
        let visited = vec![false; size];
        let order = vec![0; size];
        let low = vec![0; size];
        let ids = vec![0; size];

        StronglyConnectedComponent {
            size,
            edge,
            visited,
            order,
            low,
            ids,
            group_size: 0,
        }
    }
    fn build(&mut self) {
        let mut cnt = 0;
        let mut stack = vec![];
        for i in 0..self.size {
            if !self.visited[i] {
                self.dfs(i, &mut cnt, &mut stack);
            }
        }
        for i in 0..self.size {
            self.ids[i] = self.group_size - 1 - self.ids[i];
        }
    }
    fn dfs(&mut self, v: usize, cnt: &mut usize, stack: &mut Vec<usize>) {
        self.visited[v] = true;
        self.order[v] = *cnt;
        self.low[v] = self.order[v];
        stack.push(v);
        *cnt += 1;
        for &next in &self.edge[v] {
            if !self.visited[next] {
                self.dfs(next, cnt, stack);
                self.low[v] = self.low[v].min(self.low[next]);
            } else {
                self.low[v] = self.low[v].min(self.order[next]);
            }
        }
        if self.order[v] == self.low[v] {
            while let Some(u) = stack.pop() {
                self.order[u] = self.size;
                self.ids[u] = self.group_size;
                if v == u {
                    break;
                }
            }
            self.group_size += 1;
        }
    }
    fn scc(&mut self) -> Vec<Vec<usize>> {
        self.build();
        let mut groups = vec![vec![]; self.group_size];
        for i in 0..self.size {
            groups[self.ids[i]].push(i);
        }
        groups
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
