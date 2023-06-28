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
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, VecDeque};
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
            K: usize,
            Q: usize,
            XY: [(Usize1, usize); Q]
        }

        let mut A = vec![0; N];
        let mut mp1: BTreeMap<usize, usize> = BTreeMap::new();
        let mut mp2: BTreeMap<usize, usize> = BTreeMap::new();
        for _ in 0..K {
            *mp1.entry(0).or_default() += 1;
        }
        for _ in 0..N - K {
            *mp2.entry(0).or_default() += 1;
        }

        let mut ans = 0;

        for &(x, y) in &XY {
            let before = A[x];
            A[x] = y;
            let min = get_min(&mut mp1);
            let max = get_max(&mut mp2);
            if mp1.contains_key(&before) {
                if y > max {
                    *mp1.entry(y).or_default() += 1;
                    ans += y;
                } else {
                    *mp1.entry(max).or_default() += 1;
                    *mp2.entry(y).or_default() += 1;
                    *mp2.entry(max).or_default() -= 1;
                    ans += max;
                }
                *mp1.entry(before).or_default() -= 1;
                ans -= before;
            } else {
                if y < min {
                    *mp2.entry(y).or_default() += 1;
                } else {
                    *mp2.entry(min).or_default() += 1;
                    *mp1.entry(y).or_default() += 1;
                    *mp1.entry(min).or_default() -= 1;
                    ans += y;
                    ans -= min;
                }
                *mp2.entry(before).or_default() -= 1;
            }
            println!("{}", ans);
        }
    }
}

fn get_min(mp: &mut BTreeMap<usize, usize>) -> usize {
    let mut res = 0;
    let mut zeros = vec![];
    for (k, v) in mp.iter() {
        if *v == 0 {
            zeros.push(*k);
        } else {
            res = *k;
            break;
        }
    }
    for x in zeros {
        mp.remove(&x);
    }
    res
}

fn get_max(mp: &mut BTreeMap<usize, usize>) -> usize {
    let mut res = 0;
    let mut zeros = vec![];
    for (k, v) in mp.iter().rev() {
        if *v == 0 {
            zeros.push(*k);
        } else {
            res = *k;
            break;
        }
    }
    for x in zeros {
        mp.remove(&x);
    }
    res
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
