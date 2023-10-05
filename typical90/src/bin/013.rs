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

        let INF = 1_usize << 60;
        let mut Q = BinaryHeap::new();
        let mut dist1 = vec![INF; N];
        dist1[0] = 0;
        Q.push((Reverse(0), 0));
        while let Some((Reverse(d), pos)) = Q.pop() {
            if dist1[pos] != d {
                continue;
            }
            for &(next, cost) in &G[pos] {
                if dist1[next] > dist1[pos] + cost {
                    dist1[next] = dist1[pos] + cost;
                    Q.push((Reverse(dist1[next]), next));
                }
            }
        }
        let mut Q = BinaryHeap::new();
        let mut distN = vec![INF; N];
        distN[N - 1] = 0;
        Q.push((Reverse(0), N - 1));
        while let Some((Reverse(d), pos)) = Q.pop() {
            if distN[pos] != d {
                continue;
            }
            for &(next, cost) in &G[pos] {
                if distN[next] > distN[pos] + cost {
                    distN[next] = distN[pos] + cost;
                    Q.push((Reverse(distN[next]), next));
                }
            }
        }
        for i in 0..N {
            let ans = dist1[i] + distN[i];
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
