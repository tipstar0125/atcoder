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

const DIJ4: [(usize, usize); 4] = [(!0, 0), (0, !0), (1, 0), (0, 1)];

#[derive(Default)]
struct Solver {}
impl Solver {
    #[fastout]
    fn solve(&mut self) {
        input! {
            H: usize,
            W: usize,
            S: [Chars; H]
        }

        let f = |r: usize, c: usize| -> bool {
            for (dr, dc) in DIJ4 {
                let nr = r.wrapping_add(dr);
                let nc = c.wrapping_add(dc);
                if nr >= H || nc >= W {
                    continue;
                }
                if S[nr][nc] == '#' {
                    return false;
                }
            }
            true
        };

        let mut ans = 1;
        let mut used = vec![vec![false; W]; H];
        for i in 0..H {
            for j in 0..W {
                if used[i][j] || S[i][j] == '#' {
                    continue;
                }
                if !f(i, j) {
                    continue;
                }
                used[i][j] = true;
                let mut set = HashSet::new();
                let mut Q = VecDeque::new();
                Q.push_back((i, j));
                while let Some((pi, pj)) = Q.pop_front() {
                    set.insert((pi, pj));
                    if !f(pi, pj) {
                        continue;
                    }
                    for (di, dj) in DIJ4 {
                        let ni = pi.wrapping_add(di);
                        let nj = pj.wrapping_add(dj);
                        if ni >= H || nj >= W {
                            continue;
                        }
                        if used[ni][nj] {
                            continue;
                        }
                        if S[ni][nj] == '#' {
                            continue;
                        }
                        if f(ni, nj) {
                            used[ni][nj] = true;
                        }
                        Q.push_back((ni, nj));
                    }
                }
                ans = max!(ans, set.len());
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
