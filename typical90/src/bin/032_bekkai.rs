#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(unused_macros)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::comparison_chain)]
#![allow(clippy::nonminimal_bool)]
#![allow(clippy::neg_multiply)]
#![allow(dead_code)]
use std::{
    collections::{BTreeSet, VecDeque},
    time::Duration,
};

use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};
use superslice::Ext;

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
            A: [[usize; N]; N],
            M: usize,
            XY: [(usize, usize); M]
        }

        let mut max_length = 0;
        for j in 0..N {
            let mut max = 0;
            for i in 0..N {
                max = max!(max, A[i][j]);
            }
            max_length += max;
        }

        let mut G = vec![BTreeSet::new(); N + 1];
        for i in 1..=N {
            G[0].insert(i);
        }
        for i in 1..=N {
            for j in 1..=N {
                if i == j {
                    continue;
                }
                let mut ok = true;
                for &(x, y) in &XY {
                    if (x == i && y == j) || (x == j && y == i) {
                        ok = false;
                    }
                }
                if ok {
                    G[i].insert(j);
                    G[j].insert(i);
                }
            }
        }

        let mut dp = vec![vec![vec![]; max_length + 1]; N + 1];
        dp[0][0].push(vec![0_usize]);

        for i in 1..=N {
            for j in 0..max_length + 1 {
                if dp[i - 1][j].is_empty() {
                    continue;
                }

                for list in dp[i - 1][j].clone() {
                    let last = *list.last().unwrap();
                    for n in &G[last] {
                        if list.iter().all(|x| x != n) {
                            let mut added = list.clone();
                            added.push(*n);
                            dp[i][j + A[n - 1][i - 1]].push(added);
                        }
                    }
                }
            }
        }

        let mut ok = false;
        let mut ans = 0_isize;
        for (i, list) in dp[N].iter().enumerate() {
            if list.is_empty() {
                continue;
            }
            ok = true;
            ans = i as isize;
            break;
        }
        println!("{}", if ok { ans } else { -1 });
    }
}
fn main() {
    std::thread::Builder::new()
        .stack_size(256 * 1024 * 1024)
        .spawn(|| Solver::default().solve())
        .unwrap()
        .join()
        .unwrap();
}
