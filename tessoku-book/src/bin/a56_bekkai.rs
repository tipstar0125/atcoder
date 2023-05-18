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
            _N: usize,
            Q: usize,
            S: Chars,
            ABCD: [(usize, usize, usize, usize); Q]
        }

        let rh = RH::new(S);

        for &(a, b, c, d) in &ABCD {
            if rh.calc(a, b) == rh.calc(c, d) {
                println!("Yes");
            } else {
                println!("No");
            }
        }
    }
}

type RH = RollingHash;
struct RollingHash {
    m: usize,
    power: Vec<usize>,
    hash: Vec<usize>,
}

impl RollingHash {
    fn new(s: Vec<char>) -> Self {
        let n = s.len();
        let base = rnd::gen_range(2, 1000);
        let mut p = 1_usize;
        let mut power = vec![p];

        let mod_list = vec![1e9 as usize + 7, 998244353, 2147483647];
        let m = mod_list[rnd::gen_range(0, mod_list.len())];

        for _ in 0..n {
            p *= base;
            p %= m;
            power.push(p);
        }

        let mut h = 0_usize;
        let mut hash = vec![h];
        for i in 0..n {
            let num = (s[i] as u8 - b'a' + 1) as usize;
            h *= base;
            h += num;
            h %= m;
            hash.push(h);
        }
        RollingHash { m, power, hash }
    }
    fn calc(&self, l: usize, r: usize) -> usize {
        (self.m + self.hash[r] - self.hash[l - 1] * self.power[r - l + 1] % self.m) % self.m
    }
}

mod rnd {
    static mut S: usize = 0;
    static MAX: usize = 1e9 as usize;

    #[inline]
    pub fn init(seed: usize) {
        unsafe {
            if seed == 0 {
                let t = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs() as usize;
                S = t
            } else {
                S = seed;
            }
        }
    }
    #[inline]
    pub fn gen() -> usize {
        unsafe {
            if S == 0 {
                init(0);
            }
            S ^= S << 7;
            S ^= S >> 9;
            S
        }
    }
    #[inline]
    pub fn gen_range(a: usize, b: usize) -> usize {
        gen() % (b - a) + a
    }
    #[inline]
    pub fn gen_bool() -> bool {
        gen() & 1 == 1
    }
    #[inline]
    pub fn gen_float() -> f64 {
        ((gen() % MAX) as f64) / MAX as f64
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
