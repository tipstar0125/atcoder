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
            P: usize,
        }
        let mut C = vec![];
        let mut A = vec![];
        for _ in 0..N {
            input! {
                c: usize,
                a: [usize; K]
            }
            C.push(c);
            A.push(a);
        }

        let INF = 1_usize << 60;
        let MAX = (P + 1).pow(K as u32);
        let mut dp = vec![vec![INF; MAX]; N + 1];
        dp[0][0] = 0;
        for i in 1..=N {
            for s in 0..MAX {
                if dp[i - 1][s] >= INF {
                    continue;
                }
                dp[i][s] = min!(dp[i][s], dp[i - 1][s]);
                let mut ss = s;
                let mut ns = 0;
                let mut pp = 1;
                for k in 0..K {
                    let p = ss % (P + 1);
                    let np = min!(P, p + A[i - 1][k]);
                    ns += np * pp;
                    ss /= P + 1;
                    pp *= P + 1;
                }
                dp[i][ns] = min!(dp[i][ns], dp[i - 1][s] + C[i - 1]);
            }
        }

        if dp[N][MAX - 1] == INF {
            println!("-1");
        } else {
            println!("{}", dp[N][MAX - 1]);
        }
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
