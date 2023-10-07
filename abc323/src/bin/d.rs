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
            SC: [(usize, usize); N]
        }

        let mut heap = BinaryHeap::new();
        let mut ans = 0;
        for &(s, c) in &SC {
            heap.push((Reverse(s), c));
        }
        while !heap.is_empty() {
            let (Reverse(s), mut c) = heap.pop().unwrap();
            while !heap.is_empty() {
                let (Reverse(ss), cc) = heap.peek().unwrap();
                if *ss == s {
                    c += cc;
                    heap.pop();
                } else {
                    break;
                }
            }
            if c % 2 == 1 {
                ans += 1;
            }
            if c / 2 > 0 {
                heap.push((Reverse(2 * s), c / 2));
            }
        }
        println!("{}", ans);
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
