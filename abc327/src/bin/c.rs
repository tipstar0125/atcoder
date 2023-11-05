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
            A: [[usize; 9]; 9]
        }

        let mut ok = true;
        for i in 0..9 {
            let mut set = BTreeSet::new();
            for j in 0..9 {
                set.insert(A[i][j]);
            }
            if set.len() != 9 {
                ok = false;
            }
        }
        for j in 0..9 {
            let mut set = BTreeSet::new();
            for i in 0..9 {
                set.insert(A[i][j]);
            }
            if set.len() != 9 {
                ok = false;
            }
        }
        let mut set_vec = vec![vec![BTreeSet::new(); 3]; 3];
        for i in 0..9 {
            for j in 0..9 {
                set_vec[i / 3][j / 3].insert(A[i][j]);
            }
        }
        for i in 0..3 {
            for j in 0..3 {
                if set_vec[i][j].len() != 9 {
                    ok = false;
                }
            }
        }
        if ok {
            println!("Yes");
        } else {
            println!("No");
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
