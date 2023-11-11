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
            S: Chars
        }

        let mut ans = vec![];
        let mut L = 0;

        for s in S {
            ans.push(s);
            L += 1;
            if s == 'C' && L >= 3 && ans[L - 3] == 'A' && ans[L - 2] == 'B' {
                ans.pop();
                ans.pop();
                ans.pop();
                L -= 3;
            }
        }
        if ans.is_empty() {
            return;
        }
        println!("{}", ans.iter().join(""));
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
