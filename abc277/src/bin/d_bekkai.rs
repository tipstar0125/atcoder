#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(unused_macros)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::comparison_chain)]
#![allow(clippy::nonminimal_bool)]
#![allow(clippy::neg_multiply)]
#![allow(dead_code)]
use std::collections::BTreeMap;

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
            M: usize,
            mut A: [usize; N]
        }
        let sum = A.iter().sum::<usize>();
        let mut map = BTreeMap::new();

        for &a in &A {
            *map.entry(a).or_insert(0) += 1;
        }

        A.sort();
        A.dedup();

        let mut group = vec![];
        let mut vec = vec![];
        for i in 0..A.len() {
            if i == A.len() - 1 {
                if vec.is_empty() {
                    vec.push(A[i]);
                }
                vec.sort();
                vec.dedup();
                group.push(vec.clone());
            } else if A[i + 1] - A[i] == 1 {
                vec.push(A[i]);
                vec.push(A[i + 1]);
            } else {
                if vec.is_empty() {
                    vec.push(A[i]);
                }
                vec.sort();
                vec.dedup();
                group.push(vec.clone());
                vec = vec![];
            }
        }

        if group.len() >= 2 && group[0].contains(&0) && group[group.len() - 1].contains(&(M - 1)) {
            let mut b = group[group.len() - 1].clone();
            group[0].append(&mut b);
            group.pop();
        }

        let mut ans = 1_usize << 60;
        for g in group {
            let mut c = sum;
            for &gi in &g {
                c -= gi * map[&gi];
            }
            ans = min!(ans, c);
        }
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
