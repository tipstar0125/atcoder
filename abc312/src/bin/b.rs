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
            S: [Chars; N]
        }

        let v1 = vec!['#', '#', '#', '.'];
        let v2 = vec!['.', '.', '.', '.'];
        let v3 = vec!['.', '#', '#', '#'];

        for i in 0..N - 9 + 1 {
            for j in 0..M - 9 + 1 {
                let mut ok = true;
                for k in i..i + 3 {
                    if S[k][j..j + 4].to_vec() != v1 {
                        ok = false;
                    }
                }
                if S[i + 3][j..j + 4].to_vec() != v2 {
                    ok = false;
                }
                if S[i + 5][j + 5..j + 9].to_vec() != v2 {
                    ok = false;
                }
                for k in i + 6..i + 9 {
                    if S[k][j + 5..j + 9].to_vec() != v3 {
                        ok = false;
                    }
                }
                if ok {
                    println!("{} {}", i + 1, j + 1);
                }
            }
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
