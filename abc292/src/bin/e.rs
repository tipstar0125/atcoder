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
            UV: [(Usize1, Usize1); M]
        }

        let mut G = vec![vec![]; N];
        for &(u, v) in &UV {
            G[u].push(v);
        }

        let mut ans = 0_usize;
        for i in 0..N {
            let mut visited = vec![false; N];
            let mut Q = VecDeque::new();
            visited[i] = true;
            Q.push_back(i);
            while let Some(pos) = Q.pop_front() {
                for &next in &G[pos] {
                    if !visited[next] {
                        visited[next] = true;
                        Q.push_back(next);
                        ans += 1;
                    }
                }
            }
        }
        ans -= M;
        println!("{}", ans);
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
