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
            XYZ: [(usize, usize, usize); N]
        }

        let mut s = 0;
        let mut t = 0;
        let mut v = vec![];
        for &(x, y, z) in &XYZ {
            s += z;
            if x > y {
                t += z;
            } else {
                v.push((z, (x + y + 1) / 2 - x));
            }
        }
        if t > s / 2 {
            println!("0");
            return;
        }
        // println!("{:?}", v);

        let L = v.len();
        let ZZ = (s + 1) / 2 - t;
        let INF = 1_usize << 60;
        let mut dp = vec![vec![INF; ZZ + 1]; L + 1];
        dp[0][0] = 0;
        for i in 1..=L {
            let (hyo, cost) = v[i - 1];
            for j in 0..=ZZ {
                dp[i][j] = min!(dp[i][j], dp[i - 1][j]);
                if j + hyo < ZZ {
                    dp[i][j + hyo] = min!(dp[i][j + hyo], dp[i - 1][j] + cost);
                } else {
                    dp[i][ZZ] = min!(dp[i][ZZ], dp[i - 1][j] + cost);
                }
            }
        }
        // println!("{:?}", dp);
        let ans = dp[L][ZZ];
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
