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
            TX: [(usize, Usize1); N]
        }

        let mut enemy = vec![0; N];
        let mut catch = vec![0; N];
        for (i, &(t, x)) in TX.iter().enumerate().rev() {
            if t == 1 {
                if enemy[x] > 0 {
                    catch[i] = 1;
                    enemy[x] -= 1;
                }
            } else {
                enemy[x] += 1;
            }
        }
        let mut ans = vec![];
        let mut potion = vec![0; N];
        let mut K = 0;
        let mut K_max = 0;
        for (i, &(t, x)) in TX.iter().enumerate() {
            if t == 1 {
                ans.push(catch[i]);
                if catch[i] == 1 {
                    potion[x] += 1;
                    K += 1;
                    K_max = max!(K_max, K);
                }
            } else if potion[x] == 0 {
                println!("-1");
                return;
            } else {
                potion[x] -= 1;
                K -= 1;
            }
        }
        println!("{}", K_max);
        println!("{}", ans.iter().join(" "));
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
