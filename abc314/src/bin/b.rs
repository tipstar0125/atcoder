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
            N: usize
        }
        let mut C = vec![];
        let mut A = vec![];
        for _ in 0..N {
            input! {
                c: usize,
                a: [usize; c]
            }
            C.push(c);
            A.push(a);
        }
        input! {
            X: usize
        }

        let mut v = vec![];
        for i in 0..N {
            let c = C[i];
            let a = A[i].clone();
            if a.contains(&X) {
                v.push((c, i + 1));
            }
        }
        if v.is_empty() {
            println!("0");
        } else {
            let mut ans = vec![];
            v.sort();
            let m = v[0].0;
            for i in 0..v.len() {
                if v[i].0 == m {
                    ans.push(v[i].1);
                }
            }
            ans.sort();
            println!("{}", ans.len());
            println!("{}", ans.iter().join(" "));
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
