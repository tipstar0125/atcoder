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
            A: [usize; N]
        }

        let mut bit = BinaryIndexedTree::new(N + 1);
        let mut ans = 0_usize;
        for (i, &a) in A.iter().enumerate() {
            ans += i - bit.sum(a) as usize;
            bit.add(a, 1);
        }
        println!("{}", ans);
    }
}

#[derive(Debug, Clone)]
struct BinaryIndexedTree {
    size: usize,
    data: Vec<isize>,
}

impl BinaryIndexedTree {
    fn new(n: usize) -> Self {
        BinaryIndexedTree {
            size: n,
            data: vec![0; n],
        }
    }
    fn lsb(&self, i: usize) -> usize {
        i & i.wrapping_neg()
    }
    fn build(&mut self, v: &[isize]) {
        assert_eq!(self.size, v.len(), "size not correct!");
        self.data = v.to_vec();
        for i in 1..=self.size {
            let lsb = self.lsb(i);
            if i + lsb <= self.size {
                self.data[i + lsb - 1] += self.data[i - 1];
            }
        }
    }
    fn push(&mut self, mut x: isize) {
        self.size += 1;
        let mut d = 1;
        let k = self.lsb(self.size);
        while d != k {
            x += self.data[self.size - d - 1];
            d <<= 1;
        }
        self.data.push(x);
    }
    fn add(&mut self, i: usize, x: isize) {
        let mut idx = i + 1;
        while idx <= self.size {
            self.data[idx - 1] += x;
            idx += self.lsb(idx);
        }
    }
    //  [0, r)
    fn sum(&self, i: usize) -> isize {
        let mut ret = 0;
        let mut idx = i;
        while idx > 0 {
            ret += self.data[idx - 1];
            idx -= self.lsb(idx);
        }
        ret
    }
    // [l, r)
    fn range_sum(&self, l: usize, r: usize) -> isize {
        self.sum(r) - self.sum(l)
    }
    fn lower_bound(&self, x: isize) -> usize {
        let mut i = 0;
        let mut k = 1;
        let mut x = x;
        while k <= self.size {
            k <<= 1;
        }
        while k > 0 {
            if i + k <= self.size && self.data[i + k - 1] < x {
                x -= self.data[i + k - 1];
                i += k;
            }
            k >>= 1;
        }
        if x > 0 {
            i
        } else {
            0
        }
    }
    fn upper_bound(&self, x: isize) -> usize {
        let mut i = 0;
        let mut k = 1;
        let mut x = x;
        while k <= self.size {
            k <<= 1;
        }
        while k > 0 {
            if i + k <= self.size && self.data[i + k - 1] <= x {
                x -= self.data[i + k - 1];
                i += k;
            }
            k >>= 1;
        }
        if i < self.size {
            i
        } else {
            self.size
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
