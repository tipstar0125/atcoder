#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(unused_macros)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::comparison_chain)]
#![allow(clippy::nonminimal_bool)]
#![allow(clippy::neg_multiply)]
#![allow(dead_code)]
use std::collections::HashSet;

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
        }

        let mut set = HashSet::new();
        let mut is_ok = true;
        for _ in 0..N {
            input! {
                s: Chars
            }

            if !(s[0] == 'H' || s[0] == 'D' || s[0] == 'C' || s[0] == 'S') {
                is_ok = false;
            }
            if !(s[1] == 'A'
                || s[1] == '2'
                || s[1] == '3'
                || s[1] == '4'
                || s[1] == '5'
                || s[1] == '6'
                || s[1] == '7'
                || s[1] == '8'
                || s[1] == '9'
                || s[1] == 'T'
                || s[1] == 'J'
                || s[1] == 'Q'
                || s[1] == 'K')
            {
                is_ok = false;
            }
            set.insert(s);
        }
        if set.len() < N {
            is_ok = false;
        }

        if is_ok {
            println!("Yes");
        } else {
            println!("No");
        }
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
