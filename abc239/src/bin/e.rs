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
            X: [usize; N],
            AB: [(Usize1, Usize1); N - 1],
            VK: [(Usize1, Usize1); Q]
        }

        let mut G = vec![vec![]; N];
        let mut degs = vec![0; N];
        for &(a, b) in &AB {
            G[a].push(b);
            G[b].push(a);
            degs[a] += 1;
            degs[b] += 1;
        }

        let mut Q = VecDeque::new();
        let mut visited = vec![false; N];
        let mut kth_list = vec![vec![]; N];
        kth_list[0].push(X[0]);
        for i in 1..N {
            if degs[i] == 1 {
                Q.push_back(i);
            }
            kth_list[i].push(X[i]);
        }

        while !Q.is_empty() {
            let pos = Q.pop_front().unwrap();
            visited[pos] = true;
            for &next in &G[pos] {
                if !visited[next] {
                    let mut l = vec![];
                    kth_list[pos].sort();
                    kth_list[pos].reverse();
                    for i in 0..min!(20, kth_list[pos].len()) {
                        l.push(kth_list[pos][i]);
                    }
                    kth_list[next].extend(l);

                    degs[next] -= 1;
                    if degs[next] == 1 && next != 0 {
                        Q.push_back(next);
                    }
                }
            }
        }
        kth_list[0].sort();
        kth_list[0].reverse();

        for &(v, k) in &VK {
            println!("{}", kth_list[v][k]);
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
