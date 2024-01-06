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
            Q: usize,
        }
        let mut x: isize = 1;
        let mut y: isize = 0;
        let mut v = vec![];
        for i in (0..N).rev() {
            v.push((x + i as isize, y));
        }
        for _ in 0..Q {
            input! {
                t: usize,
            }
            if t == 1 {
                input! {
                    C: char
                }
                if C == 'R' {
                    x += 1;
                } else if C == 'L' {
                    x -= 1;
                } else if C == 'U' {
                    y += 1;
                } else {
                    y -= 1;
                }
                v.push((x, y));
            } else {
                input! {
                    p: usize
                }
                let idx = v.len() - p;
                println!("{} {}", v[idx].0, v[idx].1);
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
