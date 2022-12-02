#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(unused_macros)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::comparison_chain)]
#![allow(clippy::nonminimal_bool)]
#![allow(clippy::neg_multiply)]
#![allow(dead_code)]
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
            A: [(isize, isize,); 4]
        }

        let eval = |a: f64, b: f64, x0: f64, y0: f64, x1: f64, y1: f64| -> bool {
            (y0 > a * x0 + b && y1 > a * x1 + b) || (y0 < a * x0 + b && y1 < a * x1 + b)
        };

        let mut ok = true;
        for i in 0..4 {
            let pos0 = i;
            let pos1 = (i + 1) % 4;
            let pos2 = (i + 2) % 4;
            let pos3 = (i + 3) % 4;
            let (x0, y0) = A[pos0];
            let (x1, y1) = A[pos1];
            let (x2, y2) = A[pos2];
            let (x3, y3) = A[pos3];

            if x0 == x1 {
                ok &= (x0 > x2 && x0 > x3) || (x0 < x2 && x0 < x3);
            } else if y0 == y1 {
                ok &= (y0 > y2 && y0 > y3) || (y0 < y2 && y0 < y3);
            } else {
                let a = (y1 as f64 - y0 as f64) / (x1 as f64 - x0 as f64);
                let b = y0 as f64 - a * (x0 as f64);
                ok &= eval(a, b, x2 as f64, y2 as f64, x3 as f64, y3 as f64);
            }
        }

        println!("{}", if ok { "Yes" } else { "No" });
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
