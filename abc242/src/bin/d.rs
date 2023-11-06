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
            Q: usize
        }

        for _ in 0..Q {
            input! {
                t: usize,
                k: Usize1
            }

            let ans = f(t, k, &S);
            println!("{}", ans);
        }
    }
}

fn f(t: usize, k: usize, S: &Vec<char>) -> char {
    if t == 0 {
        return S[k];
    }
    if k == 0 {
        return g(S[0], t);
    }
    g(f(t - 1, k / 2, S), k % 2 + 1)
}

fn g(s: char, add: usize) -> char {
    let n = (s as u8 - b'A') as usize + add;
    let n = (n % 3) as u8;
    (b'A' + n) as char
}

fn main() {
    std::thread::Builder::new()
        .stack_size(128 * 1024 * 1024)
        .spawn(|| Solver::default().solve())
        .unwrap()
        .join()
        .unwrap();
}
