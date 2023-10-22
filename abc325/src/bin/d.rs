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
            TD: [(usize, usize); N]
        }

        let mut Q1 = BinaryHeap::new();
        for &(t, d) in &TD {
            Q1.push((Reverse(t), t + d));
        }

        let mut ans = 0_usize;
        let mut now = 0;
        let mut Q2 = BinaryHeap::new();
        while !Q1.is_empty() || !Q2.is_empty() {
            if Q2.is_empty() {
                let (Reverse(t), _) = Q1.peek().unwrap();
                now = *t;
            }
            while let Some((Reverse(t), d)) = Q1.peek() {
                if now == *t {
                    Q2.push(Reverse(*d));
                    Q1.pop();
                } else {
                    break;
                }
            }
            while let Some(Reverse(d)) = Q2.peek() {
                if *d < now {
                    Q2.pop();
                } else {
                    break;
                }
            }
            if !Q2.is_empty() {
                ans += 1;
                Q2.pop();
            }
            now += 1;
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
