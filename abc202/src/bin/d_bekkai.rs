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
            A: usize,
            B: usize,
            mut K: usize
        }
        let mut a = A;
        let mut b = B;
        let mut ans = vec![];
        for _ in 0..A + B {
            if a == 0 {
                ans.push('b');
                continue;
            }
            let c = nCr(a + b - 1, b);
            if K > c {
                ans.push('b');
                K -= c;
                b -= 1;
            } else {
                ans.push('a');
                a -= 1;
            }
        }
        println!("{}", ans.iter().join(""));
    }
}

fn nCr(n: usize, r: usize) -> usize {
    let mut ret = 1;
    for i in 1..=r {
        ret *= n - i + 1;
        ret /= i;
    }
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
