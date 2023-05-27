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

        let mut G = vec![vec![]; N];
        for &(u, v) in &UV {
            G[u].push(v);
            G[v].push(u);
        }

        let mut start = 0;
        for i in 0..N {
            if G[i].len() == 1 {
                start = i;
                break;
            }
        }

        let mut Q = VecDeque::new();
        let mut visited = vec![false; N];
        let mut ans = vec![];
        Q.push_back((start, 2));
        visited[start] = true;

        while !Q.is_empty() {
            let (pos, star) = Q.pop_front().unwrap();
            if star % 3 == 0 {
                ans.push(G[pos].len());
            }

            for &next in &G[pos] {
                if !visited[next] {
                    visited[next] = true;
                    Q.push_back((next, (star + 1) % 3));
                }
            }
        }

        ans.sort();
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
