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
use std::hash::Hash;
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
            D: [usize; N]
        }
        let mut ans = 0_usize;
        for i in 1..=N {
            let m = f(i);
            if m == 0 {
                continue;
            }
            for d in 1..=D[i - 1] {
                let dd = f(d);
                if dd == 0 {
                    continue;
                }
                if m == dd {
                    ans += 1;
                }
            }
        }
        println!("{}", ans);
    }
}

fn f(n: usize) -> usize {
    let mut s = n.to_string().chars().collect_vec();
    s.sort();
    s.dedup();
    if s.len() == 1 {
        return s[0] as usize;
    } else {
        return 0;
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