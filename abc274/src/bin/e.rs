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
            M: usize,
            XY: [(f64, f64); N],
            PQ: [(f64, f64); M],
        }

        let mut pos_list = vec![(0.0, 0.0)];
        pos_list.extend(XY);
        pos_list.extend(PQ);
        let INF = 9e18;
        let L = N + M + 1;
        let mut dist = vec![vec![INF; L]; L];
        for i in 0..L {
            for j in 0..L {
                let (x0, y0) = pos_list[i];
                let (x1, y1) = pos_list[j];
                let dx = x0 - x1;
                let dy = y0 - y1;
                dist[i][j] = (dx * dx + dy * dy).sqrt();
            }
        }

        let MAX = 1_usize << L;
        let mut dp = vec![vec![INF; L]; MAX];
        dp[0][0] = 0.0;

        for s in 1..MAX {
            for from in 0..L {
                for to in 0..L {
                    if s & (1 << to) == 0 {
                        continue;
                    }
                    let before_s = s ^ (1 << to);
                    let mut booster_cnt = 1;
                    let mut pos = 1 << (N + 1);
                    while pos < MAX {
                        if before_s & pos != 0 {
                            booster_cnt *= 2;
                        }
                        pos <<= 1;
                    }

                    let z = dp[before_s][from];
                    dp[s][to].chmin(z + dist[from][to] / (booster_cnt as f64));
                }
            }
        }

        let mut ans = INF;
        let mut town_set = 0;
        for i in 0..N + 1 {
            town_set += 1 << i;
        }
        for s in 0..MAX {
            if s & town_set == town_set {
                ans.chmin(dp[s][0]);
            }
        }
        println!("{}", ans);
    }
}

pub trait ChangeMinMax {
    fn chmin(&mut self, x: Self) -> bool;
    fn chmax(&mut self, x: Self) -> bool;
}

impl<T: PartialOrd> ChangeMinMax for T {
    fn chmin(&mut self, x: Self) -> bool {
        *self > x && {
            *self = x;
            true
        }
    }
    fn chmax(&mut self, x: Self) -> bool {
        *self < x && {
            *self = x;
            true
        }
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
