#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(unused_macros)]
#![allow(clippy::comparison_chain)]
#![allow(dead_code)]
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

#[fastout]
fn main() {
    input! {
        N: usize,
        S: usize,
        ab: [(usize, usize); N]
    }

    let mut dp = vec![vec![false; S + 1]; N + 1];
    dp[0][0] = true;

    for i in 1..=N {
        let (mut a, mut b) = ab[i - 1];
        if a > b {
            std::mem::swap(&mut a, &mut b);
        }

        for j in 0..=S {
            if j >= b {
                dp[i][j] = dp[i - 1][j - a] || dp[i - 1][j - b];
            } else if j >= a {
                dp[i][j] = dp[i - 1][j - a];
            }
        }
    }

    if !dp[N][S] {
        println!("No");
    } else {
        let mut ans = vec![];
        let mut j = S;
        for i in (1..=N).rev() {
            let (a, b) = ab[i - 1];
            if j >= a && dp[i - 1][j - a] {
                ans.push("H");
                j -= a;
            } else {
                ans.push("T");
                j -= b;
            }
        }
        println!("Yes");
        println!("{}", ans.iter().rev().join(""));
    }
}
