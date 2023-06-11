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
            K: usize,
            AB: [(Usize1, Usize1); M],
            PH: [(Usize1, usize); K]
        }

        let h_max = PH.iter().map(|(_, b)| b).max().unwrap();
        let mut G = vec![vec![]; N + 1];
        for &(a, b) in &AB {
            G[a].push((b, 1));
            G[b].push((a, 1));
        }
        for &(p, h) in &PH {
            G[N].push((p, h_max - h));
        }

        let INF = 1_usize << 60;
        let mut dist = vec![INF; N + 1];
        let mut Q = BinaryHeap::new();
        dist[N] = 0;
        Q.push(Reverse((0, N)));

        while !Q.is_empty() {
            let Reverse((_, pos)) = Q.pop().unwrap();
            for &(next, w) in &G[pos] {
                if dist[pos] + w < dist[next] {
                    dist[next] = dist[pos] + w;
                    Q.push(Reverse((dist[next], next)));
                }
            }
        }

        let mut ans = vec![];
        for i in 0..N {
            if dist[i] <= *h_max {
                ans.push(i + 1);
            }
        }
        println!("{}", ans.len());
        println!("{}", ans.iter().join(" "));
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
