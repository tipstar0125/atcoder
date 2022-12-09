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
            S: Chars,
            T: Chars
        }

        let mut rle_S = vec![];
        let mut cnt = 1_usize;
        for i in 1..S.len() {
            if S[i - 1] != S[i] {
                rle_S.push((S[i - 1], cnt));
                cnt = 1;
            } else {
                cnt += 1;
            }
        }
        rle_S.push((S[S.len() - 1], cnt));

        let mut rle_T = vec![];
        let mut cnt = 1_usize;
        for i in 1..T.len() {
            if T[i - 1] != T[i] {
                rle_T.push((T[i - 1], cnt));
                cnt = 1;
            } else {
                cnt += 1;
            }
        }
        rle_T.push((T[T.len() - 1], cnt));

        if rle_S.len() == rle_T.len() {
            let mut ok = true;
            for i in 0..rle_S.len() {
                let (s_str, s_num) = rle_S[i];
                let (t_str, t_num) = rle_T[i];
                if s_str != t_str {
                    ok = false;
                }
                if s_num != t_num && (s_num < 2 || s_num > t_num) {
                    ok = false;
                }
            }
            println!("{}", if ok { "Yes" } else { "No" });
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
