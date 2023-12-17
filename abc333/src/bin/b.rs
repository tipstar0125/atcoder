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
            S: Chars,
            T: Chars
        }
        let mut s1 = S[0] as u8;
        let mut s2 = S[1] as u8;
        let mut t1 = T[0] as u8;
        let mut t2 = T[1] as u8;
        if s1 > s2 {
            std::mem::swap(&mut s1, &mut s2);
        }
        if t1 > t2 {
            std::mem::swap(&mut t1, &mut t2);
        }
        let S_diff = min!(s2 - s1, 5 - (s2 - s1));
        let T_diff = min!(t2 - t1, 5 - (t2 - t1));
        if S_diff == T_diff {
            println!("Yes");
        } else {
            println!("No");
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
