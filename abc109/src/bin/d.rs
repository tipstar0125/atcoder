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
            H: usize,
            W: usize,
            mut A: [[usize; W]; H]
        }

        let mut ans = vec![];
        for i in 0..H {
            for j in 0..W - 1 {
                if A[i][j] % 2 == 1 {
                    A[i][j + 1] += 1;
                    ans.push((i, j, i, j + 1));
                }
            }
        }

        for i in 0..H - 1 {
            if A[i][W - 1] % 2 == 1 {
                A[i + 1][W - 1] += 1;
                ans.push((i, W - 1, i + 1, W - 1));
            }
        }
        println!("{}", ans.len());
        for row in &ans {
            println!("{} {} {} {}", row.0 + 1, row.1 + 1, row.2 + 1, row.3 + 1);
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
