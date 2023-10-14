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
            Q: usize,
            TX: [(usize, isize); Q]
        }

        let N = 1_usize << 20;

        let mut A = vec![-1; N];
        let mut B = vec![-1; N];

        for &(t, x) in &TX {
            if t == 1 {
                let mut idx = (x % N as isize) as usize;
                let mut v = vec![];
                while A[idx] != -1 {
                    v.push(idx);
                    idx = B[idx] as usize;
                }
                for &vi in &v {
                    B[vi] = idx as isize;
                }
                A[idx] = x;
                if B[(idx + 1) % N] == -1 {
                    B[idx] = (idx as isize + 1) % N as isize;
                } else {
                    B[idx] = B[(idx + 1) % N];
                }
            } else {
                println!("{}", A[(x % N as isize) as usize]);
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
