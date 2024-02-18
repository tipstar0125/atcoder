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
        input! {
            N: usize,
            M: usize,
            K: usize
        }
        let l = lcm(N, M);
        let n = l / N - 1;
        let m = l / M - 1;
        let mut q = K / (n + m);
        let mut r = K % (n + m);
        if r == 0 {
            r = n + m;
            q -= 1;
        }
        let mut ns = N + N * (n + 1) * q;
        let mut ms = M + M * (m + 1) * q;

        let mut cnt = 1;
        loop {
            if ns < ms {
                if cnt == r {
                    println!("{}", ns);
                    return;
                }
                ns += N;
                cnt += 1;
            } else {
                if cnt == r {
                    println!("{}", ms);
                    return;
                }
                ms += M;
                cnt += 1;
            }
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
