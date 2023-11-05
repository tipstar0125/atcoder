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
            mut P: [usize; N]
        }

        P.reverse();
        let NEG_INF = std::f64::NEG_INFINITY;
        let mut dp = vec![vec![NEG_INF; N + 1]; N + 1];
        for i in 0..=N {
            dp[i][0] = 0.0;
        }

        let mut A = vec![];
        let mut B = vec![];
        let mut C = vec![];
        let mut a = 1.0;
        let mut s = a;
        for i in 0..N {
            A.push(a);
            B.push(s);
            C.push(1200.0 / (i as f64 + 1.0).sqrt());
            a *= 0.9;
            s += a;
        }

        for i in 1..=N {
            let p = P[i - 1] as f64;
            for j in 1..=N {
                dp[i][j] = dp[i - 1][j];
                if dp[i - 1][j - 1] >= 0.0 {
                    let add = dp[i - 1][j - 1] + p * A[j - 1];
                    dp[i][j] = dp[i][j].max(add);
                }
            }
        }
        let mut ans = NEG_INF;
        for i in 1..=N {
            let r = dp[N][i] / B[i - 1] - C[i - 1];
            ans = ans.max(r);
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
