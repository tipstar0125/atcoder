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
            UV: [(Usize1, Usize1); N - 1]
        }

        let mut degs = vec![0_usize; N];
        let mut G = vec![vec![]; N];
        for &(u, v) in &UV {
            degs[u] += 1;
            degs[v] += 1;
            G[u].push(v);
            G[v].push(u);
        }

        let mut iter = 1_usize..;
        let INF = 9e18 as usize;
        let mut ans = vec![(INF, 0); N];
        let mut Q = VecDeque::new();
        let mut visited = vec![false; N];
        for (i, &d) in degs.iter().enumerate() {
            if d == 1 && i != 0 {
                let x = iter.next().unwrap();
                ans[i] = (x, x);
                Q.push_back(i);
                visited[i] = true;
            }
        }

        while !Q.is_empty() {
            let pos = Q.pop_front().unwrap();
            for &next in &G[pos] {
                if !visited[next] {
                    degs[next] -= 1;
                    ans[next].0 = min!(ans[next].0, ans[pos].0);
                    ans[next].1 = max!(ans[next].1, ans[pos].1);
                    if degs[next] == 1 && next != 0 {
                        Q.push_back(next);
                        visited[next] = true;
                    }
                }
            }
        }

        for row in &ans {
            println!("{} {}", row.0, row.1);
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
