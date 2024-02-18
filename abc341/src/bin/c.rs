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
            H: usize,
            W: usize,
            _N: usize,
            T: Chars,
            S: [Chars; H]
        }
        let mut mino = vec![(0, 0)];
        let mut r = 0_i32;
        let mut c = 0_i32;
        let mut mr = 0;
        let mut mc = 0;
        for s in T.iter() {
            if *s == 'L' {
                c -= 1;
            } else if *s == 'R' {
                c += 1;
            } else if *s == 'U' {
                r -= 1;
            } else {
                r += 1;
            }
            mino.push((r, c));
            mr = min!(mr, r);
            mc = min!(mc, c);
        }
        for i in 0..mino.len() {
            mino[i].0 -= mr;
            mino[i].1 -= mc;
        }
        mino.sort();
        mino.dedup();
        let mut ans = 0;
        for i in 1..H - 1 {
            for j in 1..W - 1 {
                let mut ok = true;
                for k in 0..mino.len() {
                    if i + mino[k].0 as usize >= H || j + mino[k].1 as usize >= W {
                        ok = false;
                        break;
                    }
                    if S[i + mino[k].0 as usize][j + mino[k].1 as usize] == '#' {
                        ok = false;
                        break;
                    }
                }
                if ok {
                    ans += 1;
                }
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
