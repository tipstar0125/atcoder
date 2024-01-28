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
        }

        let mut v = vec![(0, 0); 2 * N];
        for _ in 0..N {
            input! {
                mut a: Usize1,
                mut b: Usize1
            }
            if a > b {
                std::mem::swap(&mut a, &mut b);
            }
            v[a] = (0, b);
            v[b] = (1, a);
        }
        let mut stack = vec![];
        for i in 0..2 * N {
            if v[i].0 == 0 {
                if stack.is_empty() {
                    stack.push(v[i].1);
                } else {
                    let last = *stack.last().unwrap();
                    if last < v[i].1 {
                        println!("Yes");
                        return;
                    } else {
                        stack.push(v[i].1);
                    }
                }
            } else {
                stack.pop();
            }
        }
        println!("No");
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
