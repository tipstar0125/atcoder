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
            A: [usize; N]
        }

        let m = min!(N, 8);
        let mut vv = vec![vec![]; 200];
        for s in 1..1 << m {
            let mut c = 0;
            let mut v = vec![];
            for j in 0..m {
                if (s >> j) & 1 == 1 {
                    c += A[j];
                    c %= 200;
                    v.push(j + 1);
                }
            }
            vv[c].push(v);
        }
        for i in 0..200 {
            if vv[i].len() >= 2 {
                println!("Yes");
                println!("{} {}", vv[i][0].len(), vv[i][0].iter().join(" "));
                println!("{} {}", vv[i][1].len(), vv[i][1].iter().join(" "));
                return;
            }
        }
        println!("No");
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
