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
        }

        let mut G = vec![BTreeSet::new(); N];
        let mut cnt = N;

        for _ in 0..Q {
            input! {
                t: usize
            }

            if t == 1 {
                input! {
                    u: Usize1,
                    v: Usize1
                }

                cnt -= G[u].is_empty() as usize;
                cnt -= G[v].is_empty() as usize;
                G[u].insert(v);
                G[v].insert(u);
            } else {
                input! {
                    v: Usize1
                }

                cnt += !G[v].is_empty() as usize;
                for next in std::mem::take(&mut G[v]) {
                    G[next].remove(&v);
                    cnt += G[next].is_empty() as usize;
                }
            }
            println!("{}", cnt);
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
