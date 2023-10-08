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
            XA: isize,
            YA: isize,
            mut XB: isize,
            mut YB: isize,
            mut XC: isize,
            mut YC: isize,
        }
        XB -= XA;
        YB -= YA;
        XC -= XA;
        YC -= YA;
        if XB > XC {
            XB *= -1;
            XC *= -1;
        }
        if YB > YC {
            YB *= -1;
            YC *= -1;
        }

        let mut ans = 0;
        if XB == 0 && YB < 0 {
            XB += 1;
            XC += 1;
            ans += 1;
        } else if YB == 0 && XB < 0 {
            YB += 1;
            YC += 1;
            ans += 1;
        }

        if XB == XC {
            if YB > 0 {
                ans += YB - 1;
            } else {
                ans += (YB - 1).abs();
            }
            ans += XB.abs();
            ans += YC - YB;
        } else if YB == YC {
            if XB > 0 {
                ans += XB - 1;
            } else {
                ans += (XB - 1).abs();
            }
            ans += YB.abs();
            ans += XC - XB;
        } else {
            let mut ans1 = ans;
            let mut ans2 = ans;
            if XB > 0 {
                ans1 += XB - 1;
            } else {
                ans1 += (XB - 1).abs();
            }
            ans1 += YB.abs();
            ans1 += XC - XB;
            ans1 += 2;
            ans1 += YC - YB;

            if YB > 0 {
                ans2 += YB - 1;
            } else {
                ans2 += (YB - 1).abs();
            }
            ans2 += XB.abs();
            ans2 += YC - YB;
            ans2 += 2;
            ans2 += XC - XB;
            ans = min!(ans1, ans2);
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
