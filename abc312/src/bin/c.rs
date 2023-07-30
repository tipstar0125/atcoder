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
            mut A: [isize; N],
            mut B: [isize; M]
        }

        A.sort();
        B.sort();

        let mut ng = -1;
        let mut ok = 1e10 as isize;

        while (ok - ng) > 1 {
            let m = (ok + ng) / 2;
            if f(m, &A, &B) {
                ok = m;
            } else {
                ng = m;
            }
        }
        println!("{}", ok);
    }
}

fn f(m: isize, A: &Vec<isize>, B: &Vec<isize>) -> bool {
    let a = A.upper_bound(&m);
    let b = B.len() - B.lower_bound(&m);
    a >= b
}

fn main() {
    std::thread::Builder::new()
        .stack_size(128 * 1024 * 1024)
        .spawn(|| Solver::default().solve())
        .unwrap()
        .join()
        .unwrap();
}
