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
        let rle = run_length_encoding(S);
        if rle.len() == 3 && rle[0].0 == 'A' && rle[1].0 == 'B' && rle[2].0 == 'C' {
            println!("Yes");
            return;
        }
        if rle.len() == 2 && rle[0].0 == 'A' && rle[1].0 == 'B' {
            println!("Yes");
            return;
        }
        if rle.len() == 2 && rle[0].0 == 'B' && rle[1].0 == 'C' {
            println!("Yes");
            return;
        }
        if rle.len() == 2 && rle[0].0 == 'A' && rle[1].0 == 'C' {
            println!("Yes");
            return;
        }
        if rle.len() == 1 {
            println!("Yes");
            return;
        }

        println!("No");
    }
}

fn run_length_encoding<T: Eq>(v: Vec<T>) -> Vec<(T, usize)> {
    let mut v = v.into_iter().map(|v| (v, 1)).collect::<Vec<_>>();
    v.dedup_by(|a, b| {
        a.0 == b.0 && {
            b.1 += a.1;
            true
        }
    });
    v
}

fn main() {
    std::thread::Builder::new()
        .stack_size(128 * 1024 * 1024)
        .spawn(|| Solver::default().solve())
        .unwrap()
        .join()
        .unwrap();
}
