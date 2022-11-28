#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(unused_macros)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::comparison_chain)]
#![allow(clippy::nonminimal_bool)]
#![allow(clippy::neg_multiply)]
#![allow(dead_code)]
use std::collections::{BinaryHeap, VecDeque};

use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};

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
#[derive(Debug, Clone)]
struct UnionFind {
    parent: Vec<isize>,
    size: usize,
}
impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind {
            parent: vec![-1; n + 1],
            size: n,
        }
    }
    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] < 0 {
            return x;
        }
        let root = self.find(self.parent[x] as usize);
        self.parent[x] = root as isize;
        root
    }
    fn unite(&mut self, x: usize, y: usize) -> Option<(usize, usize)> {
        let root_x = self.find(x);
        let root_y = self.find(y);
        if root_x == root_y {
            return None;
        }
        let size_x = -self.parent[root_x];
        let size_y = -self.parent[root_y];
        self.size -= 1;
        if size_x >= size_y {
            self.parent[root_x] -= size_y;
            self.parent[root_y] = root_x as isize;
            Some((root_x, root_y))
        } else {
            self.parent[root_y] -= size_x;
            self.parent[root_x] = root_y as isize;
            Some((root_y, root_x))
        }
    }
    fn is_same(&mut self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }
    fn is_root(&mut self, x: usize) -> bool {
        self.find(x) == x
    }
    fn get_union_size(&mut self, x: usize) -> usize {
        let root = self.find(x);
        -self.parent[root] as usize
    }
    fn get_size(&self) -> usize {
        self.size
    }
}
#[derive(Default)]
struct Solver {}
impl Solver {
    #[fastout]
    fn solve(&mut self) {
        input! {
            N: usize,
            T: usize,
            AB: [(usize, usize); N-1]
        }

        let mut G = vec![vec![]; N + 1];
        for &(a, b) in &AB {
            G[a].push(b);
            G[b].push(a);
        }

        let mut Q = VecDeque::new();
        let mut dist = vec![-1_isize; N + 1];
        Q.push_back(T);
        dist[T] = 0;
        while !Q.is_empty() {
            let pos = Q.pop_front().unwrap();
            for &next in &G[pos] {
                if dist[next] == -1 {
                    dist[next] = dist[pos] + 1;
                    Q.push_back(next);
                }
            }
        }

        let mut G = vec![vec![]; N + 1];
        for &(a, b) in &AB {
            if dist[a] < dist[b] {
                G[a].push(b);
            } else {
                G[b].push(a);
            }
        }

        let mut heap = BinaryHeap::new();
        for (i, &d) in dist[1..].iter().enumerate() {
            heap.push((d, i + 1))
        }

        let mut dp = vec![0; N + 1];
        for _ in 0..N {
            let (_, pos) = heap.pop().unwrap();
            let mut max = 0;
            for &next in &G[pos] {
                max = max!(max, dp[next] + 1);
            }
            dp[pos] = max;
        }
        println!("{}", dp[1..].iter().join(" "));
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
