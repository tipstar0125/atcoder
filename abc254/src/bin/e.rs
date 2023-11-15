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
            AB: [(Usize1, Usize1); M],
            Q: usize,
        }

        let mut G = vec![vec![]; N];
        for &(a, b) in &AB {
            G[a].push(b);
            G[b].push(a);
        }

        for _ in 0..Q {
            input! {
                x: Usize1,
                k: usize
            }
            let mut dist: BTreeMap<usize, usize> = BTreeMap::new();
            let mut Q = VecDeque::new();
            let mut ans = 0_usize;
            *dist.entry(x).or_default() = 0;
            Q.push_back(x);
            ans += x + 1;
            while let Some(pos) = Q.pop_front() {
                for &next in &G[pos] {
                    if !dist.contains_key(&next) || dist[&pos] + 1 < dist[&next] {
                        if dist[&pos] + 1 > k {
                            continue;
                        }
                        *dist.entry(next).or_default() = dist[&pos] + 1;
                        ans += next + 1;
                        if dist[&next] < k {
                            Q.push_back(next);
                        }
                    }
                }
            }
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
