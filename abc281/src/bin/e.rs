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
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, VecDeque};
use superslice::Ext;

use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};

#[derive(Default)]
struct Solver {}
impl Solver {
    #[fastout]
    fn solve(&mut self) {
        input! {
            N: usize,
            M: usize,
            K: usize,
            A: [isize; N]
        }

        let mut kth_sum = KthSum::new(M, K);
        for i in 0..M {
            kth_sum.remove((0, i));
            kth_sum.add((A[i], i));
        }
        let mut ans = vec![];
        for i in 0..N - M {
            ans.push(kth_sum.S);
            kth_sum.remove((A[i], i));
            kth_sum.add((A[M + i], M + i));
        }
        ans.push(kth_sum.S);
        println!("{}", ans.iter().join(" "));
    }
}

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
            small.insert((0, i));
        }
        for i in K..N {
            large.insert((0, i));
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
        while self.small.len() > self.K {
            let p = *self.small.iter().next_back().unwrap();
            self.small.remove(&p);
            self.large.insert(p);
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
            self.S += large_min.0 - small_max.0;
        }
    }
    fn add(&mut self, p: (isize, usize)) {
        self.small.insert(p);
        self.S += p.0;
        self.balance();
    }
    fn remove(&mut self, p: (isize, usize)) {
        if self.small.remove(&p) {
            self.S -= p.0;
        } else {
            self.large.remove(&p);
        }
        self.balance();
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
