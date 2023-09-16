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
            TWS: [(usize, usize, usize); M]
        }

        let mut ans = vec![0; N];
        let mut wait_heap = BinaryHeap::new();
        let mut return_heap = BinaryHeap::new();
        for i in 0..N {
            wait_heap.push(Reverse(i));
        }

        for &(t, w, s) in &TWS {
            while !return_heap.is_empty() {
                let (Reverse(return_time), n) = return_heap.pop().unwrap();
                if return_time <= t {
                    wait_heap.push(Reverse(n));
                } else {
                    return_heap.push((Reverse(return_time), n));
                    break;
                }
            }
            if wait_heap.is_empty() {
                continue;
            }
            let Reverse(top) = wait_heap.pop().unwrap();
            ans[top] += w;
            return_heap.push((Reverse(t + s), top));
        }
        println!("{}", ans.iter().join("\n"));
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
