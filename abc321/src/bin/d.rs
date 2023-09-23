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
            P: isize,
            A: [isize; N],
            mut B: [isize; M],
        }

        let A_sum = A.iter().sum::<isize>();
        let B_sum = B.iter().sum::<isize>();
        let mut ans = A_sum * M as isize + B_sum * N as isize;
        B.sort();
        let mut S = vec![0; M + 1];
        for i in 1..=M {
            S[i] = S[i - 1] + B[i - 1];
        }

        for a in A {
            let d = P - a;
            let idx = B.upper_bound(&d);
            if idx == M {
                continue;
            }
            let cnt = M as isize - idx as isize;
            ans -= S[M] - S[idx] + a * cnt - P * cnt;
        }
        println!("{}", ans);
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
