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
            S: Chars,
            T: Chars
        }

        let mut head = vec![false; T.len() + 1];
        let mut tail = vec![false; T.len() + 1];
        head[0] = true;
        tail[T.len()] = true;
        for i in 1..=T.len() {
            if S[i - 1] == T[i - 1] || S[i - 1] == '?' || T[i - 1] == '?' {
                head[i] = true;
            } else {
                break;
            }
        }
        for i in (0..T.len()).rev() {
            if S[i + S.len() - T.len()] == T[i] || S[i + S.len() - T.len()] == '?' || T[i] == '?' {
                tail[i] = true;
            } else {
                break;
            }
        }
        for i in 0..=T.len() {
            if head[i] && tail[i] {
                println!("Yes");
            } else {
                println!("No");
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
