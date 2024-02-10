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
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, VecDeque};
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
        let mut memo = HashMap::new();
        let ans = f(N, &mut memo);
        println!("{}", ans);
    }
}

fn f(n: usize, memo: &mut HashMap<usize, usize>) -> usize {
    if n == 1 {
        return 0;
    }
    if memo.contains_key(&n) {
        return memo[&n];
    }
    let mut ret = n;
    ret += f(n / 2, memo);
    ret += f((n + 1) / 2, memo);
    memo.insert(n, ret);
    ret
}

fn main() {
    std::thread::Builder::new()
        .stack_size(128 * 1024 * 1024)
        .spawn(|| Solver::default().solve())
        .unwrap()
        .join()
        .unwrap();
}
