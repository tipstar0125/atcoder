#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(unused_macros)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::comparison_chain)]
#![allow(clippy::nonminimal_bool)]
#![allow(clippy::neg_multiply)]
#![allow(dead_code)]
use std::io::{self, stdin, stdout, BufReader, Write};

use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
    source::line::LineSource,
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
    // #[fastout]
    fn solve(&mut self) {
        let mut stdin = LineSource::new(BufReader::new(io::stdin()));
        macro_rules! input(($($tt:tt)*) => (proconio::input!(from &mut stdin, $($tt)*)));
        input! {
            N: usize
        }

        // A, B: row
        // C, D: col

        let (mut a, mut b, c, d) = (0, (N - 1) / 2, 0, N - 1);
        loop {
            println!("? {} {} {} {}", a + 1, b + 1, c + 1, d + 1);
            input! {
                T: isize
            }
            let num = b - a;
            if T == 0 {
                break;
            }
            if T == 1 && b - a == 0 {
                a += 1;
                break;
            }
            if T == -1 {
                return;
            }
            if T as usize == b + 1 - a {
                a = b + 1;
            }
            b = a + num / 2;
        }
        let ans1 = a + 1;

        let (a, b, mut c, mut d) = (0, N - 1, 0, (N - 1) / 2);
        loop {
            println!("? {} {} {} {}", a + 1, b + 1, c + 1, d + 1);
            input! {
                T: isize
            }
            let num = d - c;
            if T == 0 {
                break;
            }
            if T == 1 && d - c == 0 {
                c += 1;
                break;
            }
            if T == -1 {
                return;
            }
            if T as usize == d + 1 - c {
                c = d + 1;
            }
            d = c + num / 2;
        }
        let ans2 = c + 1;

        println!("! {} {}", ans1, ans2);
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
