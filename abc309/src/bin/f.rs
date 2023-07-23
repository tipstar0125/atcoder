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
            HWD: [[usize; 3]; N]
        }

        let mut XYZ = HWD;

        for i in 0..N {
            XYZ[i].sort();
        }
        XYZ.sort();
        XYZ.reverse();
        let mut mp: BTreeMap<usize, usize> = BTreeMap::new();
        mp.insert(XYZ[0][1], XYZ[0][2]);

        for i in 1..N {
            let x0 = XYZ[i - 1][0];
            let x1 = XYZ[i][0];
            let y1 = XYZ[i][1];
            let z1 = XYZ[i][2];

            if let Some(y) = mp.range(y1 + 1..).next() {
                if x0 > x1 && mp[y.0] > z1 {
                    println!("Yes");
                    return;
                }
            }
            if mp.contains_key(&y1) {
                *mp.entry(y1).or_default() = max!(mp[&y1], z1);
            } else {
                mp.insert(y1, z1);
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
