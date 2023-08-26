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
            ABC: [(Usize1, Usize1, usize); M]
        }

        let mut G = vec![vec![]; N];
        for &(a, b, c) in &ABC {
            G[a].push((b, c));
            G[b].push((a, c));
        }

        let mut ans = 0;
        for p in (0..N).permutations(N) {
            let mut dist = 0;
            for i in 1..N {
                let now = p[i - 1];
                let next = p[i];
                let mut ok = false;
                for &(nex, d) in &G[now] {
                    if next == nex {
                        dist += d;
                        ok = true
                    }
                }
                ans = max!(ans, dist);
                if !ok {
                    break;
                }
            }
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
