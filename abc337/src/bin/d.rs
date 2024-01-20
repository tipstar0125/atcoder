#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(unused_macros)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::comparison_chain)]
#![allow(clippy::nonminimal_bool)]
#![allow(clippy::neg_multiply)]
#![allow(dead_code)]
use amplify::confinement::Collection;
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
            K: usize,
            S: [Chars; H]
        }
        let INF = 1_usize << 60;
        let mut ans = INF;
        for i in 0..H {
            let mut Q = VecDeque::new();
            let mut cnt = 0;
            let mut used = 0;
            for j in 0..W {
                if S[i][j] == '.' {
                    cnt += 1;
                    used += 1;
                    Q.push(1);
                } else if S[i][j] == 'o' {
                    cnt += 1;
                    Q.push(0);
                } else {
                    cnt = 0;
                    used = 0;
                    Q.clear();
                }
                if cnt == K {
                    ans = min!(ans, used);
                    let t = Q.pop_front().unwrap();
                    if t == 1 {
                        used -= 1;
                    }
                    cnt -= 1;
                }
            }
        }
        for j in 0..W {
            let mut Q = VecDeque::new();
            let mut cnt = 0;
            let mut used = 0;
            for i in 0..H {
                if S[i][j] == '.' {
                    cnt += 1;
                    used += 1;
                    Q.push(1);
                } else if S[i][j] == 'o' {
                    cnt += 1;
                    Q.push(0);
                } else {
                    cnt = 0;
                    used = 0;
                    Q.clear();
                }
                if cnt == K {
                    ans = min!(ans, used);
                    let t = Q.pop_front().unwrap();
                    if t == 1 {
                        used -= 1;
                    }
                    cnt -= 1;
                }
            }
        }
        if ans == INF {
            println!("-1")
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
