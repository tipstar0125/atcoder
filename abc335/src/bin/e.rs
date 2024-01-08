#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(unused_macros)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::comparison_chain)]
#![allow(clippy::nonminimal_bool)]
#![allow(clippy::neg_multiply)]
#![allow(dead_code)]
use im_rc::HashSet;
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
            A: [usize; N],
            UV: [(Usize1, Usize1); M]
        }

        let mut G = vec![vec![]; N];
        for &(mut u, mut v) in UV.iter() {
            if A[u] > A[v] {
                std::mem::swap(&mut u, &mut v);
            }
            if A[u] == A[v] {
                G[u].push((v, 0));
                G[v].push((u, 0));
            } else {
                G[u].push((v, 1));
            }
        }
        let mut score = vec![0; N];
        let mut Q = BinaryHeap::new();
        score[0] = 1;
        Q.push((Reverse((A[0], -score[0])), 0));
        while let Some((Reverse((_, s)), pos)) = Q.pop() {
            if score[pos] != -s {
                continue;
            }
            for &(next, ns) in &G[pos] {
                if score[pos] + ns > score[next] {
                    score[next] = score[pos] + ns;
                    Q.push((Reverse((A[next], -score[next])), next));
                }
            }
        }
        println!("{}", score[N - 1]);
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
