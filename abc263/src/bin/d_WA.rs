#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(unused_macros)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::comparison_chain)]
#![allow(clippy::nonminimal_bool)]
#![allow(clippy::neg_multiply)]
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
            L: isize,
            R: isize,
            A: [isize; N]
        }

        let sum = A.iter().sum::<isize>();
        let mut left = 0;
        let mut min = sum;
        let mut S1 = vec![0; N + 1];
        for i in 1..=N {
            S1[i] = S1[i - 1] + A[i - 1];
        }
        for i in 1..=N {
            let sum1 = sum - S1[i] + L * i as isize;
            if min >= sum1 {
                left = i;
                min = sum1;
            }
        }

        let mut B = A.clone();
        for i in 0..left {
            B[i] = L;
        }

        let mut S2 = vec![0; N + 1];
        for i in 1..=N {
            S2[N - i] = S2[N - i + 1] + B[N - i];
        }

        let sum = B.iter().sum::<isize>();
        let mut min = sum;
        for i in 1..=N {
            min = min!(
                min,
                sum - S2[N - i] + min!(A[N - i] + R * (i as isize - 1), R * i as isize)
            );
        }
        println!("{}", A.iter().join(" "));
        println!("{}", S1.iter().join(" "));
        println!("{}", B.iter().join(" "));
        println!("{}", S2.iter().join(" "));
        println!("{}", min);
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
