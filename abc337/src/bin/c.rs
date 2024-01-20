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
            A: [isize; N]
        }
        let mut now = 0;
        let mut B = vec![N as isize; N + 1];
        for (i, &a) in A.iter().enumerate() {
            if a == -1 {
                now = i as isize + 1;
            }
            if a != -1 {
                B[a as usize] = i as isize + 1;
            }
        }
        let mut ans = vec![now];
        let mut cnt = 1;
        while cnt < N {
            now = B[now as usize];
            ans.push(now);
            cnt += 1;
        }
        println!("{}", ans.iter().join(" "));
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
