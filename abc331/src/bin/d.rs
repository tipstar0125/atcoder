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
            P: [Chars; N],
            ABCD: [(usize, usize, usize, usize); Q]
        }

        let mut S = vec![vec![0; 2 * N + 1]; 2 * N + 1];
        for i in 1..=2 * N {
            for j in 1..=2 * N {
                if P[(i - 1) % N][(j - 1) % N] == 'W' {
                    S[i][j] = S[i][j - 1];
                } else {
                    S[i][j] = S[i][j - 1] + 1;
                }
            }
        }
        for j in 0..=2 * N {
            for i in 1..=2 * N {
                S[i][j] += S[i - 1][j];
            }
        }
        for row in S.iter() {
            println!("{:?}", row);
        }

        for &(a, b, c, d) in &ABCD {}
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
