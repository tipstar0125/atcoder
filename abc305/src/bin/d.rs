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
            A: [usize; N],
            Q : usize,
            LR: [(usize, usize); Q]
        }

        let mut S = vec![0; N];
        for i in 1..N {
            if i % 2 == 1 {
                S[i] = S[i - 1];
            } else {
                S[i] = S[i - 1] + A[i] - A[i - 1];
            }
        }

        let f = |x: usize| -> usize {
            let pos = A.lower_bound(&x);
            if pos == 0 {
                return 0;
            }
            let mut ret = S[pos - 1];
            ret += (S[pos] - S[pos - 1]) * (x - A[pos - 1]) / (A[pos] - A[pos - 1]);
            ret
        };

        for &(l, r) in &LR {
            println!("{}", f(r) - f(l));
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
