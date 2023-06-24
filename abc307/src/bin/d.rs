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
            S: Chars
        }

        let mut stack = vec![];
        let mut memo = vec![];
        for i in 0..N {
            if S[i] == '(' {
                stack.push(i);
            } else if S[i] == ')' && !stack.is_empty() {
                let l = stack.pop().unwrap();
                memo.push((l, i));
            }
        }
        let mut v = vec![0_isize; N + 1];
        for (l, r) in memo {
            v[l] += 1;
            v[r + 1] -= 1;
        }
        let mut SS = vec![0_isize; N + 1];
        SS[0] = v[0];
        for i in 1..N + 1 {
            SS[i] = SS[i - 1] + v[i];
        }
        let mut ans = vec![];
        for i in 0..N {
            if SS[i] == 0 {
                ans.push(S[i]);
            }
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
