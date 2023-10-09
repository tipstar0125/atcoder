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
            T: usize,
        }

        for _ in 0..T {
            input! {
                N: usize,
                D: usize,
                K: usize
            }

            let T = lcm(N, D);
            let n = T / D;
            let ans = (K - 1) / n + ((K - 1) * D) % N;
            println!("{}", ans);
        }
    }
}

fn ext_gcd(a: usize, b: usize) -> (isize, isize, usize) {
    if a == 0 {
        return (0, 1, b);
    }
    let (x, y, g) = ext_gcd(b % a, a);
    (y - b as isize / a as isize * x, x, g)
}

fn lcm(a: usize, b: usize) -> usize {
    a * b / ext_gcd(a, b).2
}

fn main() {
    std::thread::Builder::new()
        .stack_size(128 * 1024 * 1024)
        .spawn(|| Solver::default().solve())
        .unwrap()
        .join()
        .unwrap();
}
