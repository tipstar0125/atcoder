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
        let mut D = vec![vec![0; N]; N];
        for i in 0..N - 1 {
            input! {
                d: [usize; N-i-1]
            }
            for (j, &w) in d.iter().enumerate() {
                D[i][i + j + 1] = w;
                D[i + j + 1][i] = w;
            }
        }

        let mut pairs = vec![];
        let mut used = vec![false; N];

        fn dfs(
            pairs: &mut Vec<(usize, usize)>,
            used: &mut Vec<bool>,
            N: usize,
            D: &Vec<Vec<usize>>,
        ) -> usize {
            if pairs.len() == N / 2 {
                let mut ret = 0;
                for (i, j) in pairs {
                    ret += D[*i][*j];
                }
                return ret;
            }
            let mut ret = 0;
            let mut p = 0;
            for i in 0..N {
                if !used[i] {
                    p = i;
                    break;
                }
            }
            used[p] = true;
            for i in 0..N {
                if !used[i] {
                    pairs.push((p, i));
                    used[i] = true;
                    ret = max!(ret, dfs(pairs, used, N, D));
                    pairs.pop();
                    used[i] = false;
                }
            }
            used[p] = false;
            ret
        }

        let mut ans = 0;
        if N % 2 == 0 {
            ans = dfs(&mut pairs, &mut used, N, &D);
        } else {
            for i in 0..N {
                used[i] = true;
                ans = max!(ans, dfs(&mut pairs, &mut used, N, &D));
                used[i] = false;
            }
        }
        println!("{}", ans);
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
