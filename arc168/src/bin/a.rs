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
            _N: usize,
            S: Chars
        }

        let rle = run_length_encoding(S);
        let mut ans = 0_usize;
        for &(k, v) in &rle {
            if k == '>' {
                ans += (1 + v) * v / 2;
            }
        }
        println!("{}", ans);
    }
}

fn run_length_encoding<T: Eq>(v: Vec<T>) -> Vec<(T, usize)> {
    let mut v = v.into_iter().map(|v| (v, 1)).collect::<Vec<_>>();
    v.dedup_by(|a, b| {
        a.0 == b.0 && {
            b.1 += a.1;
            true
        }
    });
    v
}

fn main() {
    std::thread::Builder::new()
        .stack_size(128 * 1024 * 1024)
        .spawn(|| Solver::default().solve())
        .unwrap()
        .join()
        .unwrap();
}
