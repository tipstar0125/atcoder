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
            D: usize
        }

        if N * (N - 1) / 2 < N * D {
            println!("No");
            return;
        }

        let mut ans = vec![];
        let mut cnt = 0_usize;
        let mut now = 0_usize;
        let mut delta = 1_usize;
        while cnt < N * D {
            ans.push((now, (now + delta) % N));
            now += 1;
            if now >= N {
                now = 0;
                delta += 1;
            }
            cnt += 1;
        }
        println!("Yes");
        for &(l, r) in &ans {
            println!("{} {}", l + 1, r + 1);
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
