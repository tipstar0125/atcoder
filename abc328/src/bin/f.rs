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
            Q: usize
        }

        let mut uf = WeightedUnionFind::new(N);
        let mut ans = vec![];
        for i in 0..Q {
            input! {
                a: Usize1,
                b: Usize1,
                d: isize
            }

            if !uf.is_same(a, b) {
                uf.unite(a, b, d);
                ans.push(i + 1);
            } else if uf.diff(a, b) == d {
                ans.push(i + 1);
            }
        }
        if ans.is_empty() {
            return;
        }
        println!("{}", ans.iter().join(" "));
    }
}

#[derive(Debug, Clone)]
struct WeightedUnionFind {
    parent: Vec<isize>,
    size: usize,
    diff_weight: Vec<isize>,
}

impl WeightedUnionFind {
    fn new(n: usize) -> Self {
        WeightedUnionFind {
            parent: vec![-1; n],
            size: n,
            diff_weight: vec![0_isize; n],
        }
    }
    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] < 0 {
            return x;
        }
        let root = self.find(self.parent[x] as usize);
        self.diff_weight[x] += self.diff_weight[self.parent[x] as usize];
        self.parent[x] = root as isize;
        root
    }
    fn weight(&mut self, x: usize) -> isize {
        self.find(x);
        self.diff_weight[x]
    }
    fn unite(&mut self, x: usize, y: usize, w: isize) -> Option<(usize, usize)> {
        let root_x = self.find(x);
        let root_y = self.find(y);
        if root_x == root_y {
            return None;
        }

        let adjusted_w = w + self.weight(x) - self.weight(y);
        let size_x = -self.parent[root_x];
        let size_y = -self.parent[root_y];
        self.size -= 1;
        if size_x >= size_y {
            self.diff_weight[root_y] = adjusted_w;
            self.parent[root_x] -= size_y;
            self.parent[root_y] = root_x as isize;
            Some((root_x, root_y))
        } else {
            self.diff_weight[root_x] = -adjusted_w;
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
    fn diff(&mut self, x: usize, y: usize) -> isize {
        self.weight(y) - self.weight(x)
    }
    fn get_union_size(&mut self, x: usize) -> usize {
        let root = self.find(x);
        -self.parent[root] as usize
    }
    fn get_size(&self) -> usize {
        self.size
    }
    fn roots(&self) -> Vec<usize> {
        (0..self.parent.len())
            .filter(|i| self.parent[*i] < 0)
            .collect::<Vec<usize>>()
    }
    fn members(&mut self, x: usize) -> Vec<usize> {
        let root = self.find(x);
        (0..self.parent.len())
            .filter(|i| self.find(*i) == root)
            .collect::<Vec<usize>>()
    }
    fn all_group_members(&mut self) -> BTreeMap<usize, Vec<usize>> {
        let mut groups_map: BTreeMap<usize, Vec<usize>> = BTreeMap::new();
        for x in 0..self.parent.len() {
            let r = self.find(x);
            groups_map.entry(r).or_default().push(x);
        }
        groups_map
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
