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
            H: [isize; N],
            UV: [(Usize1, Usize1); M]
        }

        let mut G = vec![vec![]; N];
        for &(u, v) in &UV {
            G[u].push(v);
            G[v].push(u);
        }

        let INF = 1_isize << 60;
        let mut cost = vec![INF; N];
        let mut Q = BinaryHeap::new();
        cost[0] = 0;
        Q.push((Reverse(cost[0]), 0));
        while let Some((Reverse(nc), pos)) = Q.pop() {
            if cost[pos] != nc {
                continue;
            }
            for &next in &G[pos] {
                let mut c = 0;
                if H[pos] < H[next] {
                    c = H[next] - H[pos];
                }
                if nc + c < cost[next] {
                    cost[next] = nc + c;
                    Q.push((Reverse(cost[next]), next));
                }
            }
        }
        let mut ans = 0_isize;
        for i in 0..N {
            ans = max!(ans, H[0] - H[i] - cost[i]);
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
