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
            T: usize,
            A: [Usize1; T]
        }

        let mut row = vec![0; N];
        let mut col = vec![0; N];
        let mut naname1 = BTreeSet::new();
        let mut i = 0;
        while naname1.len() < N {
            naname1.insert(i);
            i += N + 1;
        }
        let mut naname2 = BTreeSet::new();
        let mut i = N - 1;
        while naname2.len() < N {
            naname2.insert(i);
            i += N - 1;
        }
        let mut naname1_cnt = 0;
        let mut naname2_cnt = 0;

        for (i, a) in A.iter().enumerate() {
            row[a / N] += 1;
            if row[a / N] == N {
                println!("{}", i + 1);
                return;
            }
            col[a % N] += 1;
            if row[a / N] == N || col[a % N] == N {
                println!("{}", i + 1);
                return;
            }

            if naname1.contains(a) {
                naname1_cnt += 1;
                if naname1_cnt == N {
                    println!("{}", i + 1);
                    return;
                }
            }
            if naname2.contains(a) {
                naname2_cnt += 1;
                if naname2_cnt == N {
                    println!("{}", i + 1);
                    return;
                }
            }
        }
        println!("-1");
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
