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
            A: usize,
            X: usize,
            M: usize
        }

        let f = vec![vec![A, 1], vec![0, 1]];
        let g = mat_pow(f, X, M);
        println!("{}", g[0][1]);
    }
}

fn mat_mul(a: &[Vec<usize>], b: &[Vec<usize>], m: usize) -> Vec<Vec<usize>> {
    let n = a.len();
    let mut ret = vec![vec![0; n]; n];
    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                ret[i][j] += a[i][k] * b[k][j];
                ret[i][j] %= m;
            }
        }
    }
    ret
}

fn mat_pow(mut a: Vec<Vec<usize>>, mut b: usize, m: usize) -> Vec<Vec<usize>> {
    let n = a.len();
    let mut ret = vec![vec![0; n]; n];
    for i in 0..n {
        ret[i][i] = 1;
    }
    while b > 0 {
        if b & 1 == 1 {
            ret = mat_mul(&ret, &a, m);
        }
        a = mat_mul(&a, &a, m);
        b >>= 1;
    }
    ret
}

fn main() {
    std::thread::Builder::new()
        .stack_size(128 * 1024 * 1024)
        .spawn(|| Solver::default().solve())
        .unwrap()
        .join()
        .unwrap();
}
