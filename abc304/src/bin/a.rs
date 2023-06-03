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
            SA: [(String, usize); N]
        }

        let mut S = vec![];
        let mut A = vec![];
        for (s, a) in SA {
            S.push(s);
            A.push(a);
        }
        let pos = A.argmin().unwrap();
        for i in 0..N {
            println!("{}", S[(i + pos) % N]);
        }
    }
}

trait ArgOrd<T> {
    fn argmax(&self) -> Option<usize>;
    fn argmin(&self) -> Option<usize>;
    fn argsort(&self) -> Vec<usize>;
    fn argsort_reverse(&self) -> Vec<usize>;
}

impl<T: Ord> ArgOrd<T> for [T] {
    fn argmax(&self) -> Option<usize> {
        (0..self.len()).max_by_key(|&i| &self[i])
    }

    fn argmin(&self) -> Option<usize> {
        (0..self.len()).min_by_key(|&i| &self[i])
    }
    fn argsort(&self) -> Vec<usize> {
        (0..self.len()).sorted_by_key(|&i| &self[i]).collect_vec()
    }
    fn argsort_reverse(&self) -> Vec<usize> {
        (0..self.len())
            .sorted_by_key(|&i| std::cmp::Reverse(&self[i]))
            .collect_vec()
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
