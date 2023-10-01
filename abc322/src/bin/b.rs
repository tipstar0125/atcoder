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
            M: usize,
            S: Chars,
            T: Chars
        }

        let mut head = true;
        let mut tail = true;
        for i in 0..N {
            if S[i] != T[i] {
                head = false;
            }
        }
        for i in (0..N).rev() {
            if S[i] != T[i + M - N] {
                tail = false;
            }
        }
        if head && tail {
            println!("0");
        } else if head {
            println!("1");
        } else if tail {
            println!("2");
        } else {
            println!("3");
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
