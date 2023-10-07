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
            N: usize
        }

        let mut A = vec![vec![0; 2 * N]; 2 * N];
        for i in 0..2 * N - 1 {
            for j in i + 1..2 * N {
                input! {
                    a: usize
                }
                A[i][j] = a;
                A[j][i] = a;
            }
        }

        let mut pairs = vec![];
        let mut used = vec![false; 2 * N];

        fn dfs(
            pairs: &mut Vec<(usize, usize)>,
            N: usize,
            A: &Vec<Vec<usize>>,
            used: &mut Vec<bool>,
        ) -> usize {
            if pairs.len() == N {
                let mut ret = 0;
                for (i, j) in pairs {
                    ret ^= A[*i][*j];
                }
                return ret;
            }
            let mut ret = 0;
            let mut p = 0;
            for i in 0..2 * N {
                if !used[i] {
                    p = i;
                    break;
                }
            }
            used[p] = true;
            for i in 0..2 * N {
                if !used[i] {
                    pairs.push((p, i));
                    used[i] = true;
                    ret = max!(ret, dfs(pairs, N, A, used));
                    pairs.pop();
                    used[i] = false;
                }
            }
            used[p] = false;
            ret
        }

        println!("{}", dfs(&mut pairs, N, &A, &mut used));
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
