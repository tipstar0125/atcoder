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

        for &(l, r) in &LR {
            let pos0 = A.upper_bound(&l);
            let pos1 = A.lower_bound(&r);
            let mut ans = S[pos1] - S[pos0 - 1];
            if pos0 % 2 == 0 {
                ans -= l - A[pos0 - 1];
            }
            if pos1 % 2 == 0 {
                ans -= A[pos1] - r;
            }
            println!("{}", ans);
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
