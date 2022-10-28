#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(unused_macros)]
#![allow(clippy::comparison_chain)]
#![allow(dead_code)]
use std::mem::swap;

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
#[macro_export]
macro_rules! abs {
    ($x: expr) => {
        if $x < 0_isize {
            $x * (-1)
        } else {
            $x
        }
    };
}
#[macro_export]
macro_rules! absf {
    ($x: expr) => {
        if $x < 0.0 {
            $x * (-1.0)
        } else {
            $x
        }
    };
}

#[derive(Debug, Clone)]
struct UnionFind {
    parent: Vec<isize>,
    rank: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind {
            parent: vec![-1; n + 1],
            rank: vec![1; n + 1],
        }
    }

    fn find(&self, x: usize) -> usize {
        if self.parent[x] == -1 {
            return x;
        }
        self.find(self.parent[x] as usize)
    }

    fn unite(&mut self, mut x: usize, mut y: usize) {
        x = self.find(x);
        y = self.find(y);
        if x == y {
            return;
        }
        if self.rank[x] < self.rank[y] {
            std::mem::swap(&mut x, &mut y);
        }
        if self.rank[x] == self.rank[y] {
            self.rank[x] += 1;
        }
        self.parent[y] = x as isize;
    }

    fn is_same(&self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }
}

#[fastout]
fn main() {
    input! {
        N: usize,
        Q: usize,
        Query: [(usize, usize, usize); Q]
    }

    let mut uf = UnionFind::new(N);

    for &q in &Query {
        match q {
            (1, u, v) => {
                uf.unite(u, v);
            }
            (2, u, v) => {
                if uf.is_same(u, v) {
                    println!("Yes");
                } else {
                    println!("No");
                }
            }
            (_, _, _) => unreachable!(),
        }
    }
}
