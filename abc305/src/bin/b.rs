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
            p: char,
            q: char
        }

        let v = vec![3, 1, 4, 1, 5, 9];
        let mut s = vec![0; 7];
        for i in 1..7 {
            s[i] = s[i - 1] + v[i - 1];
        }
        let mut p_pos = p as u8 - b'A';
        let mut q_pos = q as u8 - b'A';
        if p_pos > q_pos {
            std::mem::swap(&mut p_pos, &mut q_pos);
        }
        let ans = s[q_pos as usize] - s[p_pos as usize];
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
