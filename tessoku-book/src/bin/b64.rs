#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(unused_macros)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::comparison_chain)]
#![allow(clippy::nonminimal_bool)]
#![allow(clippy::neg_multiply)]
#![allow(dead_code)]
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, VecDeque},
};

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
            M: usize,
            ABC: [(usize, usize, usize); M]
        }

        let mut G = vec![vec![]; N + 1];
        for &(a, b, c) in &ABC {
            G[a].push((b, c));
            G[b].push((a, c));
        }

        let INF = 1_usize << 60;
        let mut dist = vec![INF; N + 1];
        let mut fixed = vec![false; N + 1];
        dist[1] = 0;
        let mut Q = BinaryHeap::new();
        Q.push(Reverse((dist[1], 1)));

        while !Q.is_empty() {
            let Reverse((_, pos)) = Q.pop().unwrap();
            if fixed[pos] {
                continue;
            }
            fixed[pos] = true;

            for &next in &G[pos] {
                let (n, w) = next;
                if dist[pos] + w < dist[n] {
                    dist[n] = dist[pos] + w;
                    Q.push(Reverse((dist[n], n)));
                }
            }
        }

        let mut ans = VecDeque::new();
        let mut pos = N;
        ans.push_front(pos);
        while pos != 1 {
            for &next in &G[pos] {
                let (n, w) = next;
                if dist[pos] == dist[n] + w {
                    pos = n;
                }
            }
            ans.push_front(pos);
        }
        println!("{}", ans.iter().join(" "));
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
