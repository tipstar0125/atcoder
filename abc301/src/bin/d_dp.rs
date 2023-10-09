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
            S: Chars,
            N: usize
        }

        let mut S: VecDeque<_> = S.into_iter().collect();
        let L = 60;
        while S.len() < L {
            S.push_front('0');
        }

        let mut dp = vec![vec![-1; 2]; L + 1];
        dp[0][0] = 0;
        for i in 1..=L {
            let s = S[i - 1];
            let n = (N >> (L - i)) & 1;
            let add = 1_isize << (L - i);

            if dp[i - 1][1] != -1 {
                // smaller -> smaller
                if s == '?' || s == '1' {
                    dp[i][1] = max!(dp[i][1], dp[i - 1][1] + add);
                } else {
                    dp[i][1] = max!(dp[i][1], dp[i - 1][1]);
                }
            }
            if dp[i - 1][0] != -1 {
                // not smaller -> smaller
                if n == 1 && (s == '?' || s == '0') {
                    dp[i][1] = max!(dp[i][1], dp[i - 1][0]);
                }
                // not smaller -> not smaller
                if n == 1 && (s == '?' || s == '1') {
                    dp[i][0] = max!(dp[i][0], dp[i - 1][0] + add);
                } else if n == 0 && (s == '?' || s == '0') {
                    dp[i][0] = max!(dp[i][0], dp[i - 1][0]);
                }
            }
        }
        let ans = max!(dp[L][0], dp[L][1]);
        println!("{}", ans);
    }
}

#[macro_export]
macro_rules! max {
    ($x: expr) => ($x);
    ($x: expr, $( $y: expr ),+) => {
        std::cmp::max($x, max!($( $y ),+))
    }
}
#[macro_export]
macro_rules! min {
    ($x: expr) => ($x);
    ($x: expr, $( $y: expr ),+) => {
        std::cmp::min($x, min!($( $y ),+))
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
