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
            A: [Usize1; N]
        }

        let mut indegs = vec![0_usize; N];
        for &a in &A {
            indegs[a] += 1;
        }

        let mut Q = VecDeque::new();
        for (i, &deg) in indegs.iter().enumerate() {
            if deg == 0 {
                Q.push_back(i);
            }
        }
        let mut circle = vec![true; N];
        while let Some(pos) = Q.pop_front() {
            circle[pos] = false;
            indegs[A[pos]] -= 1;
            if indegs[A[pos]] == 0 {
                Q.push_back(A[pos]);
            }
        }

        for (i, &c) in circle.iter().enumerate() {
            if c {
                let mut ans = vec![];
                ans.push(i);
                let mut now = i;
                while A[now] != i {
                    now = A[now];
                    ans.push(now);
                }
                println!("{}", ans.len());
                println!("{}", ans.iter().map(|&x| x + 1).join(" "));
                return;
            }
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
