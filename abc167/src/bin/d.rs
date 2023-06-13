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
            K: usize,
            A: [Usize1; N]
        }

        let mut dp = vec![vec![0; N]; 60];
        dp[0] = A;
        for i in 1..60 {
            for j in 0..N {
                dp[i][j] = dp[i - 1][dp[i - 1][j]];
            }
        }

        let mut ans = 0;
        for i in (0..60).rev() {
            if (K >> i) & 1 == 1 {
                ans = dp[i][ans];
            }
        }
        println!("{}", ans + 1);
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
