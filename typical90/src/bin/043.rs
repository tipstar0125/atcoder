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

const DIJ4: [(usize, usize); 4] = [(!0, 0), (0, !0), (1, 0), (0, 1)];

#[derive(Default)]
struct Solver {}
impl Solver {
    #[fastout]
    fn solve(&mut self) {
        input! {
            H: usize,
            W: usize,
            rs: Usize1,
            cs: Usize1,
            rt: Usize1,
            ct: Usize1,
            S: [Chars; H]
        }

        let INF = 1_usize << 60;
        let mut cost = vec![vec![vec![INF; 5]; W]; H];
        let mut Q = VecDeque::new();
        cost[rs][cs][4] = 0;
        Q.push_back((rs, cs, 4));
        while let Some((r, c, dir)) = Q.pop_front() {
            for (i, &(dr, dc)) in DIJ4.iter().enumerate() {
                let nr = r.wrapping_add(dr);
                let nc = c.wrapping_add(dc);
                if nr >= H || nc >= W {
                    continue;
                }
                if S[nr][nc] == '#' {
                    continue;
                }
                if dir == 4 || i == dir {
                    if cost[nr][nc][i] > cost[r][c][dir] {
                        cost[nr][nc][i] = cost[r][c][dir];
                        Q.push_front((nr, nc, i));
                    }
                } else if cost[nr][nc][i] > cost[r][c][dir] + 1 {
                    cost[nr][nc][i] = cost[r][c][dir] + 1;
                    Q.push_back((nr, nc, i));
                }
            }
        }
        let mut ans = INF;
        for dir in 0..4 {
            ans = min!(ans, cost[rt][ct][dir]);
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
