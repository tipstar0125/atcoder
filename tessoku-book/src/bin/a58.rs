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
            Q: usize,
        }

        let mut seg = SegmentTree::new(N);

        for _ in 0..Q {
            input! {
                t: usize
            }

            if t == 1 {
                input! {
                    pos: Usize1,
                    x: isize
                }
                seg.update(pos, x);
            } else {
                input! {
                    l: usize,
                    r: usize
                }
                let ans = seg.query(l, r, 1, seg.offset + 1, 1);
                println!("{}", ans);
            }
        }
    }
}

#[derive(Debug, Clone)]
struct SegmentTree {
    offset: usize,
    data: Vec<isize>,
}

impl SegmentTree {
    fn new(n: usize) -> Self {
        let mut offset = 1;
        while offset < n {
            offset *= 2;
        }
        let data = vec![0; offset * 2];
        SegmentTree { offset, data }
    }
    fn update(&mut self, pos: usize, x: isize) {
        let mut pos = pos + self.offset;
        self.data[pos] = x;
        while pos > 1 {
            pos /= 2;
            self.data[pos] = self.data[pos * 2].max(self.data[pos * 2 + 1]);
        }
    }
    // u: current cell number
    // (a, b], (l, r]
    fn query(&self, l: usize, r: usize, a: usize, b: usize, u: usize) -> isize {
        if r <= a || b <= l {
            return -1_isize << 60;
        }
        if l <= a && b <= r {
            return self.data[u];
        }
        let m = (a + b) / 2;
        self.query(l, r, a, m, u * 2)
            .max(self.query(l, r, m, b, u * 2 + 1))
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
