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
            mut A: [usize; N]
        }

        let M = 1e8 as usize;
        let mut ans = 0;
        let mut s = A.iter().sum::<usize>() - A[0];
        for i in 1..N {
            ans += s;
            s -= A[i];
            ans += A[i - 1] * (N - i);
        }
        A.sort();
        for i in 0..N - 1 {
            let idx = A[i + 1..].lower_bound(&(M - A[i]));
            if idx == A[i + 1..].len() {
                continue;
            }
            ans -= (A[i + 1..].len() - idx) * M;
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
