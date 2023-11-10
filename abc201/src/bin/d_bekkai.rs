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
            H: usize,
            W: usize,
            A: [Chars; H]
        }

        let mut B = vec![vec![0; W]; H];
        for i in 0..H {
            for j in 0..W {
                if A[i][j] == '+' {
                    B[i][j] = 1;
                } else {
                    B[i][j] = -1;
                }
            }
        }

        let mut dp = vec![vec![0_isize; W]; H];
        let INF = 1_isize << 60;
        let NEG_INF = -INF;
        for i in (0..H).rev() {
            for j in (0..W).rev() {
                if i == H - 1 && j == W - 1 {
                    continue;
                }
                if (i + j) % 2 == 0 {
                    let mut x = NEG_INF;
                    if i + 1 < H {
                        x = max!(x, dp[i + 1][j] + B[i + 1][j]);
                    }
                    if j + 1 < W {
                        x = max!(x, dp[i][j + 1] + B[i][j + 1]);
                    }
                    dp[i][j] = x;
                } else {
                    let mut x = INF;
                    if i + 1 < H {
                        x = min!(x, dp[i + 1][j] - B[i + 1][j]);
                    }
                    if j + 1 < W {
                        x = min!(x, dp[i][j + 1] - B[i][j + 1]);
                    }
                    dp[i][j] = x;
                }
            }
        }
        if dp[0][0] > 0 {
            println!("Takahashi");
        } else if dp[0][0] < 0 {
            println!("Aoki");
        } else {
            println!("Draw");
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
