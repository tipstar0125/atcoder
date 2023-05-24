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
            X: Chars
        }

        let N = X.len();

        if N <= 2 {
            println!("{}", X.iter().join(""));
            return;
        }

        let delta = |x: usize| -> isize {
            let one = (x % 10) as isize;
            let ten = (x / 10) as isize;
            one - ten
        };

        let num: usize = X[..2].iter().join("").parse().unwrap();
        let mut bigger = false;

        for x in num..=99 {
            let d = delta(x);
            let mut before = (x % 10) as isize;
            let mut ans = x.to_string().chars().collect_vec();
            if x > num {
                bigger = true;
            }

            for i in 2..N {
                let now = before + d;
                let n = (X[i] as u8 - b'0') as isize;
                let lower = if bigger { 0 } else { n };

                if lower <= now && now <= 9 {
                    ans.push((now as u8 + b'0') as char);
                    if n < now {
                        bigger = true;
                    }
                } else {
                    break;
                }
                before += d;
            }
            if ans.len() == N {
                println!("{}", ans.iter().join(""));
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
