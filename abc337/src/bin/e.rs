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
    fn solve(&mut self) {
        let mut stdin =
            proconio::source::line::LineSource::new(std::io::BufReader::new(std::io::stdin()));
        macro_rules! input(($($tt:tt)*) => (proconio::input!(from &mut stdin, $($tt)*)));
        input! {
            N: usize
        }
        let mut b = 1;
        while 1 << b < N {
            b += 1;
        }
        println!("{}", b);
        for i in 0..b {
            let mut v = vec![];
            for j in 0..N {
                if (j >> i) & 1 == 1 {
                    v.push(j + 1);
                }
            }
            println!("{} {}", v.len(), v.iter().join(" "));
        }
        input! {
            S: Chars
        }
        let mut ans = 1;
        for (i, &s) in S.iter().enumerate() {
            if s == '1' {
                ans += 1 << i;
            }
        }
        println!("{}", ans);
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
