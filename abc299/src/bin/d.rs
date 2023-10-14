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
    input,
    marker::{Chars, Usize1},
};

macro_rules! input(($($tt:tt)*) => (
    let stdin = std::io::stdin();
    let mut stdin = proconio::source::line::LineSource::new(std::io::BufReader::new(stdin));
    proconio::input!(from &mut stdin, $($tt)*);
));

#[derive(Default)]
struct Solver {}
impl Solver {
    fn solve(&mut self) {
        input! {
            N: usize
        }

        let mut l = 1_usize;
        let mut r = N;
        while (r - l) > 1 {
            let m = (l + r) / 2;
            println!("? {}", m);
            input! {
                S: usize
            }
            if S == 0 {
                l = m;
            } else {
                r = m;
            }
        }
        println!("! {}", l);
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
