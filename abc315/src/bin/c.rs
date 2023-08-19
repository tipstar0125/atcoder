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
            FS: [(Usize1, usize); N]
        }

        let mut v = vec![vec![]; N];
        for &(f, s) in &FS {
            v[f].push(s);
        }
        let mut ans = 0;
        for i in 0..N {
            v[i].sort();
            let len = v[i].len();
            if len >= 2 {
                ans = max!(ans, v[i][len - 1] + v[i][len - 2] / 2);
            }
        }
        let mut m = vec![];
        for i in 0..N {
            if v[i].is_empty() {
                continue;
            }
            let len = v[i].len();
            m.push(v[i][len - 1]);
        }
        m.sort();
        let len = m.len();
        if len >= 2 {
            ans = max!(ans, m[len - 1] + m[len - 2]);
        }
        println!("{}", ans);
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
