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
            A: [Chars; N]
        }

        let mut B = A.clone();

        for i in 1..N {
            B[0][i] = A[0][i - 1];
        }
        for i in 1..N {
            B[i][N - 1] = A[i - 1][N - 1];
        }
        for i in 0..N - 1 {
            B[N - 1][i] = A[N - 1][i + 1];
        }
        for i in 0..N - 1 {
            B[i][0] = A[i + 1][0];
        }
        for row in B {
            println!("{}", row.iter().join(""));
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
