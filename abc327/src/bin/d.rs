#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(unused_macros)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::comparison_chain)]
#![allow(clippy::nonminimal_bool)]
#![allow(clippy::neg_multiply)]
#![allow(dead_code)]
use amplify::confinement::Collection;
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
            A: [Usize1; M],
            B: [Usize1; M]
        }

        let mut G = vec![vec![]; N];
        for i in 0..M {
            G[A[i]].push(B[i]);
            G[B[i]].push(A[i]);
        }

        let mut colors = vec![-1; N];
        let mut Q = VecDeque::new();
        for s in 0..N {
            if colors[s] != -1 {
                continue;
            }
            colors[s] = 0;
            Q.push_back(s);
            while let Some(pos) = Q.pop_front() {
                for &next in &G[pos] {
                    if colors[next] != -1 {
                        if colors[pos] == colors[next] {
                            println!("No");
                            return;
                        }
                        continue;
                    }
                    colors[next] = 1 - colors[pos];
                    Q.push_back(next);
                }
            }
        }
        println!("Yes");
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
