#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(unused_macros)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::comparison_chain)]
#![allow(clippy::nonminimal_bool)]
#![allow(clippy::neg_multiply)]
#![allow(dead_code)]
use std::collections::{BTreeMap, BTreeSet};

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
            P: usize,
            A: [usize; N]
        }

        let MOD = 1_000_000_007_usize;
        let mut cnt_map: BTreeMap<usize, usize> = BTreeMap::new();
        for i in 0..N {
            *cnt_map.entry(A[i] % MOD).or_default() += 1;
        }

        let mut pair_list = BTreeSet::new();
        let mut i = 1_usize;
        while i * i <= P {
            if P % i == 0 {
                pair_list.insert((i, P / i));
            }
            i += 1;
        }
        println!("{:?}", cnt_map);
        println!("{:?}", pair_list);

        if pair_list.is_empty() {
            let cnt_zero = *cnt_map.entry(0).or_default();
            let cnt_all = cnt_map.values().sum::<usize>();
            let cnt_others = cnt_all - cnt_zero;
            let mut ans = cnt_zero * cnt_others;
            if cnt_zero > 1 {
                ans += cnt_zero * (cnt_zero - 1) / 2;
            }
            println!("{}", ans);
        } else {
            let mut ans = 0_usize;
            for &(a, b) in &pair_list {
                if a == b {
                    let cnt_a = *cnt_map.entry(a).or_default();
                    if cnt_a > 2 {
                        ans += cnt_a * (cnt_a - 1) / 2;
                    }
                } else {
                    let cnt_a = *cnt_map.entry(a).or_default();
                    let cnt_b = *cnt_map.entry(b).or_default();
                    ans += cnt_a * cnt_b;
                }
            }
            println!("{}", ans);
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
