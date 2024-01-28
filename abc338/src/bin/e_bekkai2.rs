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
        for i in 0..N {
            input! {
                mut a: Usize1,
                mut b: Usize1
            }
            if a > b {
                std::mem::swap(&mut a, &mut b);
            }
            v[a] = (0, i);
            v[b] = (1, i);
        }
        let mut stack = vec![];
        for i in 0..2 * N {
            let (t, id) = v[i];
            if t == 0 {
                stack.push(id);
            } else {
                let last = *stack.last().unwrap();
                if id != last {
                    println!("Yes");
                    return;
                }
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
