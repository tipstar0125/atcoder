#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(unused_macros)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::comparison_chain)]
#![allow(clippy::nonminimal_bool)]
#![allow(clippy::neg_multiply)]
#![allow(dead_code)]
use itertools::Itertools;
use std::cmp::Reverse;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, VecDeque};
use superslice::Ext;

use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};

#[derive(Debug, Clone)]
struct KthSum {
    N: usize,
    K: usize,
    S: isize,
    large: BTreeSet<(isize, usize)>,
    small: BTreeSet<(isize, usize)>,
}

impl KthSum {
    fn new(N: usize, K: usize) -> Self {
        let mut large = BTreeSet::new();
        let mut small = BTreeSet::new();
        for i in 0..K {
            large.insert((0, i));
        }
        for i in K..N {
            small.insert((0, i));
        }
        KthSum {
            N,
            K,
            S: 0,
            large,
            small,
        }
    }
    fn balance(&mut self) {
        while self.large.len() > self.K {
            let p = *self.large.iter().next().unwrap();
            self.large.remove(&p);
            self.small.insert(p);
            self.S -= p.0;
        }
        if self.small.is_empty() || self.large.is_empty() {
            return;
        }
        while self.large.iter().next().unwrap().0 < self.small.iter().next_back().unwrap().0 {
            let large_min = *self.large.iter().next().unwrap();
            let small_max = *self.small.iter().next_back().unwrap();
            self.large.remove(&large_min);
            self.small.remove(&small_max);
            self.large.insert(small_max);
            self.small.insert(large_min);
            self.S += small_max.0 - large_min.0;
        }
    }
    fn add(&mut self, p: (isize, usize)) {
        self.large.insert(p);
        self.S += p.0;
        self.balance();
    }
    fn remove(&mut self, p: (isize, usize)) {
        if self.large.remove(&p) {
            self.S -= p.0;
        } else {
            self.small.remove(&p);
        }
        self.balance();
    }
}

#[derive(Default)]
struct Solver {}
impl Solver {
    #[fastout]
    fn solve(&mut self) {
        input! {
            N: usize,
            K: usize,
            Q: usize,
            XY: [(Usize1, isize); Q]
        }

        let mut A = vec![0; N];
        let mut kth_sum = KthSum::new(N, K);
        for &(x, y) in &XY {
            kth_sum.remove((A[x], x));
            kth_sum.add((y, x));
            A[x] = y;
            println!("{}", kth_sum.S);
        }
    }
}

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

fn main() {
    std::thread::Builder::new()
        .stack_size(128 * 1024 * 1024)
        .spawn(|| Solver::default().solve())
        .unwrap()
        .join()
        .unwrap();
}
