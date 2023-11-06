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
            Q: usize
        }

        for _ in 0..Q {
            input! {
                t: usize,
                mut k: Usize1
            }

            let mut si = 0;
            if t <= 60 {
                let b = 1_usize << t;
                si = k / b;
                k %= b;
            }

            let r = k.count_ones() as usize;
            let l = t - r;
            let x = (l + 2 * r + (S[si] as u8 - b'A') as usize) % 3;
            let ans = (b'A' + x as u8) as char;
            println!("{}", ans);
        }
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
