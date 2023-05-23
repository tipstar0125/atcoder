#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(unused_macros)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::comparison_chain)]
#![allow(clippy::nonminimal_bool)]
#![allow(clippy::neg_multiply)]
#![allow(dead_code)]
use itertools::Itertools;
use proconio::marker::Bytes;
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
            X: Bytes
        }

        let L = X.len();

        let mut s = 0;
        for i in 0..L {
            let num = (X[i] - b'0') as usize;
            s += num;
        }

        let mut ans = vec![0; L + 11];
        for i in 0..L + 10 {
            let mut ss = ans[i];
            ss += s;
            ans[i] = ss % 10;
            ans[i + 1] = ss / 10;
            if i < L {
                s -= (X[L - i - 1] - b'0') as usize;
            }
        }
        for i in (0..L + 11).rev() {
            if ans[i] == 0 {
                ans.pop();
            } else {
                break;
            }
        }
        ans.reverse();
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
