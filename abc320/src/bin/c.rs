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
            M: usize,
            S: [Chars; 3]
        }

        let INF = 1_usize << 60;
        let mut ans = INF;
        for i in 0..10 {
            for perm in (0..3).permutations(3) {
                let mut now = 0;
                'outer: for &p in &perm {
                    while (S[p][now % M] as u8 - b'0') as usize != i {
                        now += 1;
                        if now >= 5 * M {
                            now = INF;
                            break 'outer;
                        }
                    }
                    now += 1;
                }
                ans = min!(ans, now);
            }
        }
        if ans == INF {
            println!("-1");
        } else {
            println!("{}", ans - 1);
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
