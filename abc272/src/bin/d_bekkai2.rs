#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(unused_macros)]
#![allow(clippy::comparison_chain)]
#![allow(clippy::nonminimal_bool)]
#![allow(clippy::neg_multiply)]
#![allow(dead_code)]
use std::collections::VecDeque;

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

#[derive(Default)]
struct Solver {}
impl Solver {
    #[fastout]
    fn solve(&mut self) {
        input! {
            N:usize,
            M:usize
        }

        let mut dist = vec![];
        for i in 0..=N {
            for j in 0..=N {
                if i * i + j * j == M {
                    dist.push((i as isize, j as isize));
                    dist.push((i as isize * -1, j as isize));
                    dist.push((i as isize, j as isize * -1));
                    dist.push((i as isize * -1, j as isize * -1));
                }
            }
        }

        let mut ans = vec![vec![-1; N]; N];
        ans[0][0] = 0;
        let mut Q = VecDeque::new();
        for &(a, b) in &dist {
            Q.push_back(((0, 0), (a, b)));
        }

        while !Q.is_empty() {
            let ((x0, y0), (c, d)) = Q.pop_front().unwrap();

            if is_area(x0 + c, y0 + d, N, N) && ans[(x0 + c) as usize][(y0 + d) as usize] == -1 {
                ans[(x0 + c) as usize][(y0 + d) as usize] = ans[x0 as usize][y0 as usize] + 1;
                for &(a, b) in &dist {
                    Q.push_back(((x0 + c, y0 + d), (a, b)));
                }
            }
        }
        for row in &ans {
            println!("{}", row.iter().join(" "));
        }
    }
}

fn main() {
    std::thread::Builder::new()
        .stack_size(64 * 1024 * 1024)
        .spawn(|| Solver::default().solve())
        .unwrap()
        .join()
        .unwrap();
}

fn is_area(i: isize, j: isize, h: usize, w: usize) -> bool {
    i >= 0 && j >= 0 && i < h as isize && j < w as isize
}
