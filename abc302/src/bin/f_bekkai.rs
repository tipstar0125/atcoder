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
        }

        let mut G = vec![vec![]; N + M];
        for i in 0..N {
            input! {
                A: usize,
                S: [Usize1; A]
            }
            for &s in &S {
                G[i].push((s + N, 1));
                G[s + N].push((i, 0));
            }
        }

        let mut Q = BinaryHeap::new();
        let INF = 1_usize << 60;
        let mut dist = vec![INF; N + M];
        dist[N] = 0;
        Q.push(Reverse((dist[N], N)));
        while !Q.is_empty() {
            let Reverse((_, pos)) = Q.pop().unwrap();
            for &(next, d) in &G[pos] {
                if dist[pos] + d < dist[next] {
                    dist[next] = dist[pos] + d;
                    Q.push(Reverse((dist[next], next)));
                }
            }
        }

        if dist[N + M - 1] >= INF {
            println!("-1");
        } else {
            let ans = dist[N + M - 1] - 1;
            println!("{}", ans);
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
