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
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashSet, VecDeque};
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
            T: Chars,
            N: usize,
        }
        let mut S = vec![];
        for _ in 0..N {
            input! {
                a: usize,
                s:[Chars; a]
            }
            S.push(s);
        }
        let INF = 1_usize << 60;
        let mut dp = vec![INF; T.len() + 1];
        dp[0] = 0;
        for i in 0..N {
            let mut ndp = vec![INF; T.len() + 1];
            for j in 0..=T.len() {
                if dp[j] == INF {
                    continue;
                }
                ndp[j] = min!(ndp[j], dp[j]);
                for s in S[i].iter() {
                    if j + s.len() > T.len() {
                        continue;
                    }
                    let mut ok = true;
                    for k in 0..s.len() {
                        if s[k] != T[j + k] {
                            ok = false;
                            break;
                        }
                    }
                    if ok {
                        ndp[j + s.len()] = min!(ndp[j + s.len()], dp[j] + 1);
                    }
                }
            }
            dp = ndp;
        }
        let ans = dp[T.len()];
        if ans == INF {
            println!("-1");
        } else {
            println!("{}", ans);
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
