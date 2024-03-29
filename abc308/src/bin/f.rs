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
            mut P: [usize; N],
            L: [usize; M],
            D: [usize; M],
        }

        let mut heap = BinaryHeap::new();
        for (&l, &d) in L.iter().zip(D.iter()) {
            heap.push(Reverse((l, d)));
        }

        P.sort();
        let mut ans = 0_usize;
        let mut heap_max = BinaryHeap::new();
        for &p in &P {
            ans += p;
            while let Some(Reverse((l, d))) = heap.pop() {
                if p >= l {
                    heap_max.push(d);
                } else {
                    heap.push(Reverse((l, d)));
                    break;
                }
            }
            if let Some(x) = heap_max.pop() {
                ans -= x;
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
