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
            L: usize,
            A: [usize; N],
            B: [usize; M],
            CD: [(Usize1, Usize1); L]
        }

        let mut AA = vec![];
        for (i, &a) in A.iter().enumerate() {
            AA.push((a, i));
        }
        AA.sort();
        AA.reverse();
        let mut BB = vec![];
        for (i, &b) in B.iter().enumerate() {
            BB.push((b, i));
        }
        BB.sort();
        BB.reverse();

        let mut set = BTreeSet::new();
        for &cd in &CD {
            set.insert(cd);
        }

        let mut ans = 0_usize;
        for i in 0..N {
            for j in 0..M {
                let (a, c) = AA[i];
                let (b, d) = BB[j];
                if set.contains(&(c, d)) {
                    continue;
                }
                ans = max!(ans, a + b);
                break;
            }
        }
        for j in 0..M {
            for i in 0..N {
                let (a, c) = AA[i];
                let (b, d) = BB[j];
                if set.contains(&(c, d)) {
                    continue;
                }
                ans = max!(ans, a + b);
                break;
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
