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
            A: usize,
            B: usize,
            mut K: usize
        }

        let mut dp = vec![vec![0; B + 1]; A + 1];
        dp[0][0] = 1;
        for i in 0..=A {
            for j in 0..=B {
                if j > 0 {
                    dp[i][j] += dp[i][j - 1];
                }
                if i > 0 {
                    dp[i][j] += dp[i - 1][j];
                }
            }
        }

        let mut a = A;
        let mut b = B;
        let mut ans = vec![];
        for _ in 0..A + B {
            if a == 0 {
                ans.push('b');
                continue;
            }
            let c = dp[a - 1][b];
            if K > c {
                ans.push('b');
                K -= c;
                b -= 1;
            } else {
                ans.push('a');
                a -= 1;
            }
        }
        println!("{}", ans.iter().join(""));
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
