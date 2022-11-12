#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(unused_macros)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::comparison_chain)]
#![allow(clippy::nonminimal_bool)]
#![allow(clippy::neg_multiply)]
#![allow(dead_code)]
use std::collections::{BTreeMap, BTreeSet};

use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};

#[macro_export]
macro_rules! max {
    (x: expr) => (x);
    (x: expr, $( y: expr ),+) => {
        std::cmp::max(x, max!($( y ),+))
    }
}
#[macro_export]
macro_rules! min {
    (x: expr) => (x);
    (x: expr, $( y: expr ),+) => {
        std::cmp::min(x, min!($( y ),+))
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
    fn unite(&mut self, x: usize, y: usize) {
        let root_x = self.find(x);
        let root_y = self.find(y);
        if root_x == root_y {
            return;
        }
        let size_x = -self.parent[root_x];
        let size_y = -self.parent[root_y];
        if size_x >= size_y {
            self.parent[root_x] -= size_y;
            self.parent[root_y] = root_x as isize;
        } else {
            self.parent[root_y] -= size_x;
            self.parent[root_x] = root_y as isize;
        }
        self.size -= 1;
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
            AB: [(usize, usize); N]
        }

        let mut G = BTreeMap::new();

        for &(a, b) in &AB {
            G.entry(a).or_insert(Vec::new()).push(b);
            G.entry(b).or_insert(Vec::new()).push(a);
        }

        let mut visited = BTreeSet::new();
        let mut ans = 1;
        if !G.contains_key(&1) {
            println!("1");
            return;
        }
        dfs(1, &mut G, &mut visited, &mut ans);
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

fn dfs(
    pos: usize,
    G: &mut BTreeMap<usize, Vec<usize>>,
    visited: &mut BTreeSet<usize>,
    ans: &mut usize,
) {
    visited.insert(pos);
    if pos > *ans {
        *ans = pos;
    }
    for next in G[&pos].clone() {
        if !visited.contains(&next) {
            dfs(next, G, visited, ans);
        }
    }
}
