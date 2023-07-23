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
            N: usize,
            AB: [(Usize1, Usize1); N]
        }

        let mut S = vec![vec![0; W]; H];
        for &(a, b) in &AB {
            S[a][b] = 1;
        }
        let mut T = S.clone();
        for i in 0..H {
            for j in 1..W {
                T[i][j] += T[i][j - 1];
            }
        }
        for j in 0..W {
            for i in 1..H {
                T[i][j] += T[i - 1][j];
            }
        }

        let f = |m: usize, r: usize, c: usize| -> bool {
            if r + m - 1 >= H || c + m - 1 >= W {
                return false;
            }
            let mut cnt = T[r + m - 1][c + m - 1];
            if r != 0 {
                cnt -= T[r - 1][c + m - 1];
            }
            if c != 0 {
                cnt -= T[r + m - 1][c - 1];
            }
            if r != 0 && c != 0 {
                cnt += T[r - 1][c - 1];
            }
            cnt == 0
        };

        let mut ans = 0_usize;
        for i in 0..H {
            for j in 0..W {
                if S[i][j] == 1 {
                    continue;
                }
                let mut ok = 1;
                let mut ng = 3001;
                while ng - ok > 1 {
                    let m = (ok + ng) / 2;
                    if f(m, i, j) {
                        ok = m;
                    } else {
                        ng = m;
                    }
                }
                ans += ok;
            }
        }
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
