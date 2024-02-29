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
            LDKCAB: [(usize, usize, usize, usize, Usize1, Usize1); M]
        }
        let mut degs = vec![0; N];
        let mut G = vec![vec![]; N];
        for &(l, d, k, c, a, b) in &LDKCAB {
            if a == N - 1 {
                continue;
            }
            G[b].push((l, d, k, c, a));
            degs[a] += 1;
        }
        let mut ans = vec![0; N];
        ans[N - 1] = std::usize::MAX;
        let mut Q = VecDeque::new();
        for i in 0..N {
            if degs[i] == 0 {
                Q.push_back(i);
            }
        }
        while let Some(pos) = Q.pop_front() {
            for &(l, d, k, c, nxt) in &G[pos] {
                degs[nxt] -= 1;
                let mut ok = 0;
                let mut ng = k;
                while ng - ok > 1 {
                    let m = (ok + ng) / 2;
                    if l + m * d + c <= ans[pos] {
                        ok = m;
                    } else {
                        ng = m;
                    }
                }
                if l + ok * d + c <= ans[pos] {
                    ans[nxt] = max!(ans[nxt], l + ok * d);
                }
                if degs[nxt] == 0 {
                    Q.push_back(nxt);
                }
            }
        }
        for i in 0..N - 1 {
            if ans[i] == 0 {
                println!("Unreachable");
            } else {
                println!("{}", ans[i]);
            }
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
